// 予約済みキーワード
pub static RESERVED_WORDS: &[&str] = &[
    "if", "else", "while", "for", "break", "continue", "i32", "i64", "f32", "f64", "u32", "u64",
    "type", "let", "l", "var", "v", "fn", "mut", "loop", "=", "+", "++", "-", "--", "+=", "-=",
    "*", "*=", "/", "/=", "{", "}", "[", "]", "mod", "use", "bool", "struct", "enum", "%", "&",
    "&=", "|", "|=", "^", "~", "^=",
];
pub static SYSTEM_TYPE_NAME:&[&str] = &[
    "u8","u16","u32","u64","i8","i16","i32","i64","f32","f64","string",
    "pu8","pu16","pu32","pu64","pi8","pi16","pi32","pi64","pf32","pf64","pstring"
];
#[derive(Debug, Clone)]
pub enum SystemValue {
    Usize(usize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    Bool(bool),
    Function(String,Vec<(String,String)>,String,Vec<SystemValue>),
    Array(Vec<SystemValue>),
    Pointer(Box<SystemValue>),
    Tuple(Vec<SystemValue>),
    Null,
}
#[derive(Debug, Clone)]
pub enum TypeError {
    Mismatch(String, String),  // 期待される型と実際の型が一致しない
    UnknownType(String),       // 型が存在しない場合のエラー
}
