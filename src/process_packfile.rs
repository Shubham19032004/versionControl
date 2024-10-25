// Implemented using https://dev.to/calebsander/git-internals-part-2-packfiles-1jg8 as a reference

use anyhow::{bail, Context, Result};
use std::io::{ErrorKind, Read};

const VARINT_ENCODING_BITS: u8 = 7;
const VARINT_CONTINUE_FLAG: u8 = 1 << VARINT_ENCODING_BITS;
const TYPE_BITS: u8 = 3;
const TYPE_BYTE_SIZE_BITS: u8 = VARINT_ENCODING_BITS - TYPE_BITS;
const COPY_INSTRUCTION_FLAG: u8 = 1 << 7;
const COPY_SIZE_BYTES: u8 = 3;
const COPY_ZERO_SIZE: usize = 0x10000;
const COPY_OFFSET_BYTES: u8 = 4;

pub fn read_varint_byte<R: Read>(packfile_reader: &mut R) -> Result<(u8, bool)> {
    let mut bytes: [u8; 1] = [0];

    packfile_reader.read_exact(&mut bytes)?;

    let [byte] = bytes;
    let value = byte & !VARINT_CONTINUE_FLAG;
    let more_bytes = byte & VARINT_CONTINUE_FLAG != 0;

    Ok((value, more_bytes))
}

pub fn read_size_encoding<R: Read>(packfile_reader: &mut R) -> Result<usize> {
    let mut value = 0;
    let mut length = 0;

    loop {
        let (byte_value, more_bytes) = read_varint_byte(packfile_reader)?;

        value |= (byte_value as usize) << length;

        if !more_bytes {
            return Ok(value);
        }

        length += VARINT_ENCODING_BITS;
    }
}

pub fn keep_bits(value: usize, bits: u8) -> usize {
    value & ((1 << bits) - 1)
}

pub fn read_type_and_size<R: Read>(packfile_reader: &mut R) -> Result<ObjectType> {
    let value = read_size_encoding(packfile_reader)?;
    let object_type = keep_bits(value >> TYPE_BYTE_SIZE_BITS, TYPE_BITS) as u8;
    let size = keep_bits(value, TYPE_BYTE_SIZE_BITS)
        | (value >> VARINT_ENCODING_BITS << TYPE_BYTE_SIZE_BITS);

    Ok(ObjectType::new(object_type, size))
}

pub fn read_size<R: Read>(packfile_reader: &mut R) -> Result<usize> {
    let value = read_size_encoding(packfile_reader).context("reading size encoding")?;
    let size = keep_bits(value, TYPE_BYTE_SIZE_BITS);

    Ok(size)
}

#[derive(Debug)]
pub enum ObjectType {
    Commit(usize),
    Tree(usize),
    Blob(usize),
    Tag(usize),
    OfsDelta(usize),
    RefDelta(usize),
    Unknown,
}

impl ObjectType {
    pub fn new(object_type: u8, size: usize) -> Self {
        match object_type {
            1 => Self::Commit(size),
            2 => Self::Tree(size),
            3 => Self::Blob(size),
            4 => Self::Tag(size),
            6 => Self::OfsDelta(size),
            7 => Self::RefDelta(size),
            _ => {
                eprintln!("Error, object type must be 1-7, got {object_type}");
                Self::Unknown
            }
        }
    }

    pub fn get_size(&self) -> Option<usize> {
        Some(*match self {
            ObjectType::Commit(size) => size,
            ObjectType::Tree(size) => size,
            ObjectType::Blob(size) => size,
            ObjectType::Tag(size) => size,
            ObjectType::OfsDelta(size) => size,
            ObjectType::RefDelta(size) => size,
            ObjectType::Unknown => unreachable!(),
        })
    }

    pub fn get_type(&self) -> &'static str {
        match self {
            Self::Commit(_) => "commit",
            Self::Tree(_) => "tree",
            Self::Unknown => unreachable!(),
            ObjectType::Blob(_) => "blob",
            ObjectType::Tag(_) => "tag",
            ObjectType::OfsDelta(_) => "ofsdelta",
            ObjectType::RefDelta(_) => "refdelta",
        }
    }

    pub fn is_delta(&self) -> bool {
        matches!(self, ObjectType::OfsDelta(_) | ObjectType::RefDelta(_))
    }
}

pub fn read_bytes<R: Read, const N: usize>(stream: &mut R) -> std::io::Result<[u8; N]> {
    let mut bytes = [0; N];

    stream.read_exact(&mut bytes)?;

    Ok(bytes)
}

pub fn read_partial_int<R: Read>(
    stream: &mut R,
    bytes: u8,
    present_bytes: &mut u8,
) -> Result<usize> {
    let mut value = 0;

    for byte_index in 0..bytes {
        if *present_bytes & 1 != 0 {
            let [byte] = read_bytes(stream).context("reading one byte")?;

            value |= (byte as usize) << (byte_index * 8);
        }

        *present_bytes >>= 1;
    }

    Ok(value)
}

pub fn apply_delta_instruction<R: Read>(
    stream: &mut R,
    base: &[u8],
    result: &mut Vec<u8>,
) -> Result<bool> {
    let instruction = match read_bytes(stream) {
        Ok([instruction]) => instruction,
        Err(error) if error.kind() == ErrorKind::UnexpectedEof => return Ok(false),
        Err(error) => return Err(error.into()),
    };

    if instruction & COPY_INSTRUCTION_FLAG == 0 {
        if instruction == 0 {
            eprintln!("got bad instruction :(");
            bail!("invalid data instruction");
        }

        let mut data = vec![0; instruction as usize];

        stream.read_exact(&mut data).context("reading data")?;
        result.extend_from_slice(&data);
    } else {
        // copy instruction
        let mut nonzero_bytes = instruction;
        let offset = read_partial_int(stream, COPY_OFFSET_BYTES, &mut nonzero_bytes)
            .context("getting offset")?;
        let mut size = read_partial_int(stream, COPY_SIZE_BYTES, &mut nonzero_bytes)
            .context("getting size")?;

        if size == 0 {
            size = COPY_ZERO_SIZE;
        }

        let base_data = base
            .get(offset..(offset + size))
            .context("invalid copy instruction")?;

        result.extend_from_slice(base_data);
    }

    Ok(true)
}
