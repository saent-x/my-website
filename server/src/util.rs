use std::path::Path;
use std::error::Error;
use std::fs;


pub fn read_md_file(path: &Path) -> Result<String, Box<dyn Error>>{
    let md_string = fs::read_to_string(path.to_str().unwrap())?;

    Ok(md_string)
}