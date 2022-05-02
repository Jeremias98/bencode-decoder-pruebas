use std::collections::HashMap;

use super::buffer::Buffer;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum BencodeElement {
    String(String),
    Bytes(Vec<u8>),
    Integer(i64),
    List(Vec<BencodeElement>),
    Dictionary(HashMap<String, BencodeElement>),
    RawDictionary(HashMap<Vec<u8>, BencodeElement>),
}

pub struct Decoder;

const DICTIONARY_PREFIX: u8 = b'd';
const NUMBER_PREFIX: u8 = b'i';
const LIST_PREFIX: u8 = b'l';
const STRING_DELIMITER: u8 = b':';
const POSTFIX: u8 = b'e';

impl Decoder {

    pub fn new() -> Decoder {
        Decoder {  }
    }
    
    pub fn parse(&self, buffer: &mut Buffer) -> Option<BencodeElement> {
        let item = buffer.get();
        match item {
            Some(&DICTIONARY_PREFIX) => {
                buffer.next(1);
                self.parse_dictionary(buffer)
            },
            _ => None
        }
    }

    fn parse_dictionary(&self, buffer: &mut Buffer) -> Option<BencodeElement> {

        if buffer.get() == None {
            return None
        }

        let dictionary: HashMap<String, BencodeElement> = HashMap::new();
        while buffer.get() != Some(&POSTFIX) {
            let key = self.parse_string(buffer);
            buffer.next(1);
        }

        Some(BencodeElement::Dictionary(dictionary))
    }

    fn parse_string(&self, buffer: &mut Buffer) -> Result<BencodeElement, String> {

        if buffer.get() == None {
            return Err("Sin valor".to_string())
        }

        let length_bytes: Vec<u8> = self.get_bytes(buffer, STRING_DELIMITER);

        let lenght: usize = String::from_utf8(length_bytes).unwrap_or("0".to_string()).parse().unwrap_or(0);
        
        // salto los :
        buffer.next(1);

        let word = buffer.take_bytes(lenght);

        let decoded_string = String::from_utf8(word).unwrap_or("".to_string());

        Ok(BencodeElement::String(decoded_string))
    }

    fn get_bytes(&self, buffer: &mut Buffer, delimiter: u8) -> Vec<u8> {
        let mut length_bytes: Vec<u8> = Vec::new();

        while buffer.get() != Some(&delimiter) {
            match buffer.get() {
                Some(value) => length_bytes.push(*value),
                None => break
            }
            buffer.next(1);
        }

        length_bytes
    }

    fn parse_int(&self, buffer: &mut Buffer) -> Result<BencodeElement, String> {
        let int_bytes = self.get_bytes(buffer, POSTFIX);
        let decoded_string = String::from_utf8(int_bytes).map(|err| err.to_string()).unwrap();
        println!("{}", decoded_string);
        let element = BencodeElement::Integer(decoded_string.parse().unwrap_or(0));
        Ok(element)
    }

}


#[cfg(test)]
mod decode_test {

    use super::*;

    #[test]
    fn parse_string_ok() {
        let bytes = "4:pepe".as_bytes();
        let expected = BencodeElement::String("pepe".to_string());
        let mut buffer = Buffer::new(bytes);
        let decoder = Decoder::new();
        assert_eq!(decoder.parse_string(&mut buffer).unwrap(), expected);
    }
    #[test]
    fn parse_int_ok() {
        let bytes = "420e".as_bytes();
        let expected = BencodeElement::Integer(420);
        let mut buffer = Buffer::new(bytes);
        let decoder = Decoder::new();
        assert_eq!(decoder.parse_int(&mut buffer).unwrap(), expected);
    }
}
