mod types;
mod traits;

use lalrpop_util::*;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use types::*;
use traits::*;
use log::info;
lalrpop_mod!(pub calculator);
lazy_static!{
    static ref VARIABLES:Mutex<HashMap<String,(Option<String>,SystemValue)>>  = Mutex::new(HashMap::new());
}
pub fn parse_to_system_value(s:&str)->SystemValue{
            if let Ok(n) = s.parse::<i8>() {
                SystemValue::I8(n)
            } else if let Ok(n) = s.parse::<i16>() {
                SystemValue::I16(n)
            } else if let Ok(n) = s.parse::<i32>() {
                SystemValue::I32(n)
            } else if let Ok(n) = s.parse::<i64>() {
                SystemValue::I64(n)
            } else if let Ok(n) = s.parse::<u8>() {
                SystemValue::U8(n)
            } else if let Ok(n) = s.parse::<u16>() {
                SystemValue::U16(n)
            } else if let Ok(n) = s.parse::<u32>() {
                SystemValue::U32(n)
            } else if let Ok(n) = s.parse::<u64>() {
                SystemValue::U64(n)
            } else if let Ok(n) = s.parse::<f32>() {
                SystemValue::F32(n)
            } else if let Ok(n) = s.parse::<f64>() {
                SystemValue::F64(n)
            } else {
                SystemValue::String(s.to_string())
            }
}
/*
pub fn check_type(expected_type: &str, value: &SystemValue) -> Result<(), TypeError> {
    // システムの型名に存在するか確認
    /*
    if !SYSTEM_TYPE_NAME.contains(&expected_type) {
        return Err(TypeError::UnknownType(expected_type.to_string()));
    }*/

    // 型が一致するか確認
    match (expected_type, value) {
        ("u8", SystemValue::U8(_)) => Ok(()),
        ("u16", SystemValue::U16(_)) => Ok(()),
        ("u32", SystemValue::U32(_)) => Ok(()),
        ("u64", SystemValue::U64(_)) => Ok(()),
        ("i8", SystemValue::I8(_)) => Ok(()),
        ("i16", SystemValue::I16(_)) => Ok(()),
        ("i32", SystemValue::I32(_)) => Ok(()),
        ("i64", SystemValue::I64(_)) => Ok(()),
        ("f32", SystemValue::F32(_)) => Ok(()),
        ("f64", SystemValue::F64(_)) => Ok(()),
        ("string", SystemValue::String(_)) => Ok(()),
        ("bool", SystemValue::Bool(_)) => Ok(()),
        ("array", SystemValue::Array(_)) => Ok(()),
        ("pointer", SystemValue::Pointer(_)) => Ok(()),
        ("tuple", SystemValue::Tuple(_)) => Ok(()),
        ("null", SystemValue::Null) => Ok(()),
        // ポインタ型のチェック
        ("pu8", SystemValue::Pointer(boxed_val)) => check_type("u8", boxed_val),
        ("pu16", SystemValue::Pointer(boxed_val)) => check_type("u16", boxed_val),
        ("pu32", SystemValue::Pointer(boxed_val)) => check_type("u32", boxed_val),
        ("pu64", SystemValue::Pointer(boxed_val)) => check_type("u64", boxed_val),
        ("pi8", SystemValue::Pointer(boxed_val)) => check_type("i8", boxed_val),
        ("pi16", SystemValue::Pointer(boxed_val)) => check_type("i16", boxed_val),
        ("pi32", SystemValue::Pointer(boxed_val)) => check_type("i32", boxed_val),
        ("pi64", SystemValue::Pointer(boxed_val)) => check_type("i64", boxed_val),
        ("pf32", SystemValue::Pointer(boxed_val)) => check_type("f32", boxed_val),
        ("pf64", SystemValue::Pointer(boxed_val)) => check_type("f64", boxed_val),
        ("pstring", SystemValue::Pointer(boxed_val)) => check_type("string", boxed_val),
        // 配列型のチェック
        (expected, SystemValue::Array(elems)) if expected.starts_with("array<") => {
            let inner_type = &expected[6..expected.len()-1]; // "array<inner_type>"からinner_typeを抽出
            for elem in elems {
                check_type(inner_type, elem)?;
            }
            Ok(())
        },
        // 型が一致しない場合
        (_, value) => Err(TypeError::Mismatch(expected_type.to_string(), format!("{:?}", value))),
    }
}
*/
pub fn check_type(expected_type: &str, value: &SystemValue) -> Result<(), TypeError> {
    // システムの型名に存在するか確認
/*    if !SYSTEM_TYPE_NAME.contains(&expected_type) {
        return Err(TypeError::UnknownType(expected_type.to_string()));
    }
*/
    // 型が一致するか確認
    match (expected_type, value) {
        ("u8", SystemValue::U8(_)) => Ok(()),
        ("u16", SystemValue::U16(_)) => Ok(()),
        ("u32", SystemValue::U32(_)) => Ok(()),
        ("u64", SystemValue::U64(_)) => Ok(()),
        ("i8", SystemValue::I8(_)) => Ok(()),
        ("i16", SystemValue::I16(_)) => Ok(()),
        ("i32", SystemValue::I32(_)) => Ok(()),
        ("i64", SystemValue::I64(_)) => Ok(()),
        ("f32", SystemValue::F32(_)) => Ok(()),
        ("f64", SystemValue::F64(_)) => Ok(()),
        ("string", SystemValue::String(_)) => Ok(()),
        ("bool", SystemValue::Bool(_)) => Ok(()),
        ("array", SystemValue::Array(_)) => Ok(()),
        ("pointer", SystemValue::Pointer(_)) => Ok(()),
        ("tuple", SystemValue::Tuple(_)) => Ok(()),
        ("null", SystemValue::Null) => Ok(()),
        // ポインタ型のチェック
        (expected, SystemValue::Pointer(boxed_val)) => check_type(expected, boxed_val),
        // 配列型のチェック
        (expected, SystemValue::Array(elems)) if expected.starts_with("array<") => {
            let inner_type = &expected[6..expected.len()-1]; // "array<inner_type>"からinner_typeを抽出
            for elem in elems {
                check_type(inner_type, elem)?;
            }
            Ok(())
        },
        // 型が一致しない場合
        (_, value) => Err(TypeError::Mismatch(expected_type.to_string(), format!("{:?}", value))),
    }
}
pub fn infer_type(value: &SystemValue) -> Result<String, TypeError> {
    match value {
        SystemValue::U8(_) => Ok("u8".to_string()),
        SystemValue::U16(_) => Ok("u16".to_string()),
        SystemValue::U32(_) => Ok("u32".to_string()),
        SystemValue::U64(_) => Ok("u64".to_string()),
        SystemValue::I8(_) => Ok("i8".to_string()),
        SystemValue::I16(_) => Ok("i16".to_string()),
        SystemValue::I32(_) => Ok("i32".to_string()),
        SystemValue::I64(_) => Ok("i64".to_string()),
        SystemValue::F32(_) => Ok("f32".to_string()),
        SystemValue::F64(_) => Ok("f64".to_string()),
        SystemValue::String(_) => Ok("string".to_string()),
        SystemValue::Bool(_) => Ok("bool".to_string()),
        SystemValue::Array(elems) => {
            if let Some(first_elem) = elems.first() {
                let inner_type = infer_type(first_elem)?;
                Ok(format!("array<{}>", inner_type))
            } else {
                Err(TypeError::UnknownType("Empty array".to_string()))
            }
        },
        SystemValue::Pointer(inner) => {
            let inner_type = infer_type(inner)?;
            Ok(format!("{}", inner_type))
        },
        SystemValue::Tuple(_) => Ok("tuple".to_string()),
        SystemValue::Null => Ok("null".to_string()),
        _ => Err(TypeError::UnknownType("Unknown SystemValue type".to_string())),
    }
}

fn main() {
    std::env::set_var("RUST_LOG","debug");
    env_logger::init();

    {
        let mut vars = VARIABLES.lock().unwrap();
        vars.insert("x".to_string(),(None,SystemValue::F32(1919.1)));
    }
    let stmt = "let a =100;";
    let res = calculator::StatementsParser::new().parse(stmt);
    match res {
        Ok(value) => {
            info!("Result: {:?}",value);
            info!("Global variables: {:?}",VARIABLES.lock().unwrap());
        },
        Err(e) => println!("Error: {:?}",e),
    }
    std::env::remove_var("RUST_LOG");
}
