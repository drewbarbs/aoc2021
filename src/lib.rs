use std::env;
use std::error::Error;
use std::fs;

pub fn get_input_string() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        Err(format!(
            "Usage: {} <input file>",
            args.get(0).unwrap_or(&"prog".into())
        ))?;
    }

    fs::read_to_string(&args[1]).map_err(|e| e.into())
}
