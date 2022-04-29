use super::buffer::Buffer;

pub struct Decoder;

const DICTIONARY_PREFIX: u8 = b'd';
const NUMBER_PREFIX: u8 = b'i';
const POSTFIX: u8 = b'e';

impl Decoder {
    
    pub fn parse(buffer: &mut Buffer) {
        while !buffer.is_empty() {
            match buffer.get() {
                Some(&DICTIONARY_PREFIX) => { buffer.next(1); },
                None => (),
                _ => (),
            }
            buffer.next(1);
        }
    }

    // fn parse_dictionary(buffer: &mut Buffer) {
    //     //let mut entries = Vec::new();
    //     while buffer.get() != Some(&POSTFIX) {
    //         match buffer.get() {
    //             Some()
    //         }
    //         buffer.next(1);
    //     }
    // }

    // fn parse_string(buffer: &mut Buffer) {
        
    // }
}
