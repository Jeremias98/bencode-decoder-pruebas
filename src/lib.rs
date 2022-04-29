pub mod tools;

use tools::{arg_parser::ArgParser, file_reader::FileReader, buffer::Buffer, decoder::Decoder};

pub fn run() -> Result<(), String> {
    let params = ArgParser::get_params();

    if params.len() < 2 {
        return Err("Debe proveer una ruta para leer el archivo .torrent".to_string());
    }

    let content = FileReader::read(&params[0])?;

    let mut buffer = Buffer::new(&content);

    Decoder::parse(&mut buffer);

    Ok(())
}