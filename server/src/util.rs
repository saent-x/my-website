use std::error::Error;
use std::fs;
use std::path::Path;

use crate::prelude::ResponseStatusType;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;


pub fn gen_uuid() -> String {
    let mut uuid_buffer: [u8; 45] = Uuid::encode_buffer();
    let bp_uuid: &mut str = Uuid::new_v4().simple().encode_lower(&mut uuid_buffer);

    bp_uuid.to_string()
}

pub fn gen_response<T>(response_status_type: ResponseStatusType, data: T) -> Value
where
    T: Serialize,
{
    match response_status_type {
        ResponseStatusType::Success(code) => {
            json!(
                {
                    "status": "success",
                    "code": code,
                    "data":  data
                }
            )
        }
        ResponseStatusType::Error(code) => {
            json!(
                {
                    "status": "error",
                    "code": code,
                    "data":  data
                }
            )
        }
    }
}
