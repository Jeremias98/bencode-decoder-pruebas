pub mod tools;

use tools::{arg_parser::ArgParser,file_reader::FileReader, buffer::Buffer, decoder::{Decoder} };

use crate::tools::decoder::BencodeElement;

pub fn run() -> Result<(), String> {
    let params = ArgParser::get_params();

    if params.len() < 2 {
        return Err("Debe proveer una ruta para leer el archivo .torrent".to_string());
    }

    let content = FileReader::read(&params[1])?;

    let mut buffer = Buffer::new(&content);

    let decoder = Decoder::new();
    let result = decoder.parse(&mut buffer)?;
    
    match result {
        BencodeElement::Dictionary(dict) => println!("{:?}", dict["announce"]),
        _ => println!("ppe")
    }

    Ok(())
}