use std::env;

pub struct ArgParser;

impl ArgParser {
    /// Devuelve un vector de strings con los argumentos pasados por consola.
    pub fn get_params() -> Vec<String> {
        env::args().collect()
    }
}
