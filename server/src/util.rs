use std::path::Path;
use std::error::Error;
use std::fs;

use uuid::Uuid;


pub fn read_md_file(path: &Path) -> Result<String, Box<dyn Error>>{
    let md_string = fs::read_to_string(path.to_str().unwrap())?;

    Ok(md_string)
}

pub fn gen_uuid() -> String{
    let mut uuid_buffer: [u8; 45] = Uuid::encode_buffer();
    let bp_uuid: &mut str = Uuid::new_v4().simple().encode_lower(&mut uuid_buffer);

    bp_uuid.to_string()
}