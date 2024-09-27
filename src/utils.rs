use std::io::Read;

use flate2::read::ZlibDecoder;

pub fn get_object_directory_name(hash:&str)->String{
   hash[0..2].to_owned()
}
pub fn get_object_file_name(hash:&str)->String{
    hash[2..].to_owned()
 }
 pub fn decompress(bytes:&[u8])->Vec<u8>{
    let mut decoder=ZlibDecoder::new(bytes);
    let mut result=vec![];
    decoder.read_to_end(&mut result).unwrap();
    result
 }
 
#[cfg(test)]
mod test{
    use std::{io::Write};

    use flate2::{write::ZlibEncoder, Compression};

    use super::*;
    #[test]
    fn should_get_object_directory_name_from_hash(){
        let hash="213123123123123123112fdfwe23d2d2";
        let expected_name="21";
        let name=get_object_directory_name(hash);
        assert_eq!(name,expected_name)
    }
    #[test]
    fn should_get_object_file_name_from_hash(){
        let hash="213123123123123123112fdfwe23d2d2";
        let expected_name="3123123123123123112fdfwe23d2d2";
        let name=get_object_file_name(hash);
        assert_eq!(name,expected_name)
    }
    #[test]
    fn should_decompress(){
        let hash="213123123123123123112fdfwe23d2d2";

        let mut encoder=ZlibEncoder::new(vec![],Compression::default());
        encoder.write_all(hash.as_bytes()).unwrap();        
        let compressed=encoder.finish().unwrap();
        let de_compressed=decompress(&compressed); 
        assert_eq!(hash.as_bytes(),de_compressed)
    }
}