use crate::types::SystemValue;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Shl, Shr, Sub};
// Addトレイトの実装
impl Add for SystemValue {
    type Output = Result<SystemValue, String>;

    fn add(self, other: SystemValue) -> Self::Output {
        match (self, other) {
            (SystemValue::U8(a), SystemValue::U8(b)) => Ok(SystemValue::U8(a + b)),
            (SystemValue::U16(a), SystemValue::U16(b)) => Ok(SystemValue::U16(a + b)),
            (SystemValue::U32(a), SystemValue::U32(b)) => Ok(SystemValue::U32(a + b)),

            (SystemValue::U64(a), SystemValue::U64(b)) => Ok(SystemValue::U64(a + b)),
            (SystemValue::I8(a), SystemValue::I8(b)) => Ok(SystemValue::I8(a + b)),
            (SystemValue::I16(a), SystemValue::I16(b)) => Ok(SystemValue::I16(a + b)),
            (SystemValue::I32(a), SystemValue::I32(b)) => Ok(SystemValue::I32(a + b)),
            (SystemValue::I64(a), SystemValue::I64(b)) => Ok(SystemValue::I64(a + b)),
            (SystemValue::F32(a), SystemValue::F32(b)) => Ok(SystemValue::F32(a + b)),
            (SystemValue::F64(a), SystemValue::F64(b)) => Ok(SystemValue::F64(a + b)),
            (SystemValue::String(a), SystemValue::String(b)) => Ok(SystemValue::String(a + &b)),
            _ => Err("加算できない型です".to_string()),
        }
    }
}

// Subトレイトの実装
impl Sub for SystemValue {
    type Output = Result<SystemValue, String>;

    fn sub(self, other: SystemValue) -> Self::Output {
        match (self, other) {
            (SystemValue::U8(a), SystemValue::U8(b)) => Ok(SystemValue::U8(a - b)),
            (SystemValue::U16(a), SystemValue::U16(b)) => Ok(SystemValue::U16(a - b)),
            (SystemValue::U32(a), SystemValue::U32(b)) => Ok(SystemValue::U32(a - b)),
            (SystemValue::U64(a), SystemValue::U64(b)) => Ok(SystemValue::U64(a - b)),
            (SystemValue::I8(a), SystemValue::I8(b)) => Ok(SystemValue::I8(a - b)),
            (SystemValue::I16(a), SystemValue::I16(b)) => Ok(SystemValue::I16(a - b)),
            (SystemValue::I32(a), SystemValue::I32(b)) => Ok(SystemValue::I32(a - b)),

            (SystemValue::I64(a), SystemValue::I64(b)) => Ok(SystemValue::I64(a - b)),
            (SystemValue::F64(a), SystemValue::F64(b)) => Ok(SystemValue::F64(a - b)),
            _ => Err("減算できない型です".to_string()),
        }
    }
}

// Mulトレイトの実装
impl Mul for SystemValue {
    type Output = Result<SystemValue, String>;

    fn mul(self, other: SystemValue) -> Self::Output {
        match (self, other) {
            (SystemValue::U8(a), SystemValue::U8(b)) => Ok(SystemValue::U8(a * b)),
            (SystemValue::U16(a), SystemValue::U16(b)) => Ok(SystemValue::U16(a * b)),
            (SystemValue::U32(a), SystemValue::U32(b)) => Ok(SystemValue::U32(a * b)),
            (SystemValue::U64(a), SystemValue::U64(b)) => Ok(SystemValue::U64(a * b)),
            (SystemValue::I8(a), SystemValue::I8(b)) => Ok(SystemValue::I8(a * b)),
            (SystemValue::I16(a), SystemValue::I16(b)) => Ok(SystemValue::I16(a * b)),
            (SystemValue::I32(a), SystemValue::I32(b)) => Ok(SystemValue::I32(a * b)),
            (SystemValue::I64(a), SystemValue::I64(b)) => Ok(SystemValue::I64(a * b)),
            (SystemValue::F32(a), SystemValue::F32(b)) => Ok(SystemValue::F32(a * b)),
            (SystemValue::F64(a), SystemValue::F64(b)) => Ok(SystemValue::F64(a * b)),

            _ => Err("乗算できない型です".to_string()),
        }
    }
}

// Divトレイトの実装
impl Div for SystemValue {
    type Output = Result<SystemValue, String>;

    fn div(self, other: SystemValue) -> Self::Output {
        match (self, other) {
            (SystemValue::U8(a), SystemValue::U8(b)) => {
                if b == 0 {
                    Err("ゼロで割ることはできません".to_string())
                } else {
                    Ok(SystemValue::U8(a / b))
                }
            }

            (SystemValue::U16(a), SystemValue::U16(b)) => {
                if b == 0 {
                    Err("ゼロで割ることはできません".to_string())
                } else {
                    Ok(SystemValue::U16(a / b))
                }
            }

            (SystemValue::U32(a), SystemValue::U32(b)) => {
                if b == 0 {
                    Err("ゼロで割ることはできません".to_string())
                } else {
                    Ok(SystemValue::U32(a / b))
                }
            }

            (SystemValue::U64(a), SystemValue::U64(b)) => {
                if b == 0 {
                    Err("ゼロで割ることはできません".to_string())
                } else {
                    Ok(SystemValue::U64(a / b))
                }
            }

            (SystemValue::I8(a), SystemValue::I8(b)) => {
                if b == 0 {
                    Err("ゼロで割ることはできません".to_string())
                } else {
                    Ok(SystemValue::I8(a / b))
                }
            }

            (SystemValue::I16(a), SystemValue::I16(b)) => {
                if b == 0 {
                    Err("ゼロで割ることはできません".to_string())
                } else {
                    Ok(SystemValue::I16(a / b))
                }
            }

            (SystemValue::I32(a), SystemValue::I32(b)) => {
                if b == 0 {
                    Err("ゼロで割ることはできません".to_string())
                } else {
                    Ok(SystemValue::I32(a / b))
                }
            }

            (SystemValue::I64(a), SystemValue::I64(b)) => {
                if b == 0 {
                    Err("ゼロで割ることはできません".to_string())
                } else {
                    Ok(SystemValue::I64(a / b))
                }
            }
            (SystemValue::F64(a), SystemValue::F64(b)) => {
                if b == 0.0 {
                    Err("ゼロで割ることはできません".to_string())
                } else {
                    Ok(SystemValue::F64(a / b))
                }
            }
            _ => Err("除算できない型です".to_string()),
        }
    }
}

// BitAndトレイトの実装
impl BitAnd for SystemValue {
    type Output = Result<SystemValue, String>;

    fn bitand(self, other: SystemValue) -> Self::Output {
        match (self, other) {
            (SystemValue::I8(a), SystemValue::I8(b)) => Ok(SystemValue::I8(a & b)),
            (SystemValue::I16(a), SystemValue::I16(b)) => Ok(SystemValue::I16(a & b)),
            (SystemValue::I32(a), SystemValue::I32(b)) => Ok(SystemValue::I32(a & b)),
            (SystemValue::I64(a), SystemValue::I64(b)) => Ok(SystemValue::I64(a & b)),

            (SystemValue::U8(a), SystemValue::U8(b)) => Ok(SystemValue::U8(a & b)),
            (SystemValue::U16(a), SystemValue::U16(b)) => Ok(SystemValue::U16(a & b)),
            (SystemValue::U32(a), SystemValue::U32(b)) => Ok(SystemValue::U32(a & b)),
            (SystemValue::U64(a), SystemValue::U64(b)) => Ok(SystemValue::U64(a & b)),
            _ => Err("ビットAND演算できない型です".to_string()),
        }
    }
}

// BitOrトレイトの実装
impl BitOr for SystemValue {
    type Output = Result<SystemValue, String>;

    fn bitor(self, other: SystemValue) -> Self::Output {
        match (self, other) {
            (SystemValue::I8(a), SystemValue::I8(b)) => Ok(SystemValue::I8(a | b)),
            (SystemValue::I16(a), SystemValue::I16(b)) => Ok(SystemValue::I16(a | b)),
            (SystemValue::I32(a), SystemValue::I32(b)) => Ok(SystemValue::I32(a | b)),
            (SystemValue::I64(a), SystemValue::I64(b)) => Ok(SystemValue::I64(a | b)),

            (SystemValue::U8(a), SystemValue::U8(b)) => Ok(SystemValue::U8(a | b)),
            (SystemValue::U16(a), SystemValue::U16(b)) => Ok(SystemValue::U16(a | b)),
            (SystemValue::U32(a), SystemValue::U32(b)) => Ok(SystemValue::U32(a | b)),
            (SystemValue::U64(a), SystemValue::U64(b)) => Ok(SystemValue::U64(a | b)),

            _ => Err("ビットOR演算できない型です".to_string()),
        }
    }
}

// BitXorトレイトの実装
impl BitXor for SystemValue {
    type Output = Result<SystemValue, String>;

    fn bitxor(self, other: SystemValue) -> Self::Output {
        match (self, other) {
            (SystemValue::I8(a), SystemValue::I8(b)) => Ok(SystemValue::I8(a ^ b)),
            (SystemValue::I16(a), SystemValue::I16(b)) => Ok(SystemValue::I16(a ^ b)),
            (SystemValue::I32(a), SystemValue::I32(b)) => Ok(SystemValue::I32(a ^ b)),
            (SystemValue::I64(a), SystemValue::I64(b)) => Ok(SystemValue::I64(a ^ b)),

            (SystemValue::U8(a), SystemValue::U8(b)) => Ok(SystemValue::U8(a ^ b)),
            (SystemValue::U16(a), SystemValue::U16(b)) => Ok(SystemValue::U16(a ^ b)),
            (SystemValue::U32(a), SystemValue::U32(b)) => Ok(SystemValue::U32(a ^ b)),
            (SystemValue::U64(a), SystemValue::U64(b)) => Ok(SystemValue::U64(a ^ b)),

            _ => Err("ビットXOR演算できない型です".to_string()),
        }
    }
}

// Notトレイトの実装
impl Not for SystemValue {
    type Output = Result<SystemValue, String>;

    fn not(self) -> Self::Output {
        match self {
            (SystemValue::I8(a)) => Ok(SystemValue::I8(!a)),
            (SystemValue::I16(a)) => Ok(SystemValue::I16(!a)),
            (SystemValue::I32(a)) => Ok(SystemValue::I32(!a)),
            (SystemValue::I64(a)) => Ok(SystemValue::I64(!a)),

            (SystemValue::U8(a)) => Ok(SystemValue::U8(!a)),
            (SystemValue::U16(a)) => Ok(SystemValue::U16(!a)),
            (SystemValue::U32(a)) => Ok(SystemValue::U32(!a)),
            (SystemValue::U64(a)) => Ok(SystemValue::U64(!a)),

            _ => Err("ビット反転できない型です".to_string()),
        }
    }
}

impl Shl<u32> for SystemValue {
    type Output = Result<SystemValue, String>;

    fn shl(self, rhs: u32) -> Self::Output {
        match self {
            (SystemValue::I8(a)) => Ok(SystemValue::I8(a << rhs)),
            (SystemValue::I16(a)) => Ok(SystemValue::I16(a << rhs)),
            (SystemValue::I32(a)) => Ok(SystemValue::I32(a << rhs)),
            (SystemValue::I64(a)) => Ok(SystemValue::I64(a << rhs)),

            (SystemValue::U8(a)) => Ok(SystemValue::U8(a << rhs)),
            (SystemValue::U16(a)) => Ok(SystemValue::U16(a << rhs)),
            (SystemValue::U32(a)) => Ok(SystemValue::U32(a << rhs)),
            (SystemValue::U64(a)) => Ok(SystemValue::U64(a << rhs)),

            _ => Err("ビットシフト左できない型です".to_string()),
        }
    }
}

impl Shr<u32> for SystemValue {
    type Output = Result<SystemValue, String>;

    fn shr(self, rhs: u32) -> Self::Output {
        match self {
            (SystemValue::I8(a)) => Ok(SystemValue::I8(a << rhs)),
            (SystemValue::I16(a)) => Ok(SystemValue::I16(a >> rhs)),
            (SystemValue::I32(a)) => Ok(SystemValue::I32(a >> rhs)),
            (SystemValue::I64(a)) => Ok(SystemValue::I64(a >> rhs)),

            (SystemValue::U8(a)) => Ok(SystemValue::U8(a >> rhs)),
            (SystemValue::U16(a)) => Ok(SystemValue::U16(a >> rhs)),
            (SystemValue::U32(a)) => Ok(SystemValue::U32(a >> rhs)),
            (SystemValue::U64(a)) => Ok(SystemValue::U64(a >> rhs)),

            _ => Err("ビットシフト右できない型です".to_string()),
        }
    }
}

impl Shl<SystemValue> for SystemValue {
    type Output = Result<SystemValue, String>;

    fn shl(self, rhs: SystemValue) -> Self::Output {
        match rhs {
            SystemValue::I32(b) => {
                if b < 0 {
                    return Err("シフト量は負であってはいけません".to_string());
                }
                let shift = b as u32;

                match self {
                    SystemValue::I8(a) => Ok(SystemValue::I8(a << shift)),
                    SystemValue::I16(a) => Ok(SystemValue::I16(a << shift)),
                    SystemValue::I32(a) => Ok(SystemValue::I32(a << shift)),
                    SystemValue::I64(a) => Ok(SystemValue::I64(a << shift)),
                    SystemValue::U8(a) => Ok(SystemValue::U8(a << shift)),
                    SystemValue::U16(a) => Ok(SystemValue::U16(a << shift)),
                    SystemValue::U32(a) => Ok(SystemValue::U32(a << shift)),
                    SystemValue::U64(a) => Ok(SystemValue::U64(a << shift)),
                    _ => Err("ビットシフト左できない型です".to_string()),
                }
            }
            _ => Err("シフト量はI32型でなければなりません".to_string()),
        }
    }
}

impl Shr<SystemValue> for SystemValue {
    type Output = Result<SystemValue, String>;

    fn shr(self, rhs: SystemValue) -> Self::Output {
        match rhs {
            SystemValue::I32(b) => {
                if b < 0 {
                    return Err("シフト量は負であってはいけません".to_string());
                }
                let shift = b as u32;

                match self {
                    SystemValue::I8(a) => Ok(SystemValue::I8(a >> shift)),
                    SystemValue::I16(a) => Ok(SystemValue::I16(a >> shift)),
                    SystemValue::I32(a) => Ok(SystemValue::I32(a >> shift)),
                    SystemValue::I64(a) => Ok(SystemValue::I64(a >> shift)),
                    SystemValue::U8(a) => Ok(SystemValue::U8(a >> shift)),
                    SystemValue::U16(a) => Ok(SystemValue::U16(a >> shift)),
                    SystemValue::U32(a) => Ok(SystemValue::U32(a >> shift)),
                    SystemValue::U64(a) => Ok(SystemValue::U64(a >> shift)),
                    _ => Err("ビットシフト右できない型です".to_string()),
                }
            }
            _ => Err("シフト量はI32型でなければなりません".to_string()),
        }
    }
}
impl From<usize> for SystemValue {
    fn from(value: usize) -> Self {
        SystemValue::Usize(value)
    }
}

impl From<u8> for SystemValue {
    fn from(value: u8) -> Self {
        SystemValue::U8(value)
    }
}

impl From<u16> for SystemValue {
    fn from(value: u16) -> Self {
        SystemValue::U16(value)
    }
}

impl From<u32> for SystemValue {
    fn from(value: u32) -> Self {
        SystemValue::U32(value)
    }
}

impl From<u64> for SystemValue {
    fn from(value: u64) -> Self {
        SystemValue::U64(value)
    }
}

impl From<i8> for SystemValue {
    fn from(value: i8) -> Self {
        SystemValue::I8(value)
    }
}

impl From<i16> for SystemValue {
    fn from(value: i16) -> Self {
        SystemValue::I16(value)
    }
}

impl From<i32> for SystemValue {
    fn from(value: i32) -> Self {
        SystemValue::I32(value)
    }
}

impl From<i64> for SystemValue {
    fn from(value: i64) -> Self {
        SystemValue::I64(value)
    }
}

impl From<f32> for SystemValue {
    fn from(value: f32) -> Self {
        SystemValue::F32(value)
    }
}

impl From<f64> for SystemValue {
    fn from(value: f64) -> Self {
        SystemValue::F64(value)
    }
}

impl From<String> for SystemValue {
    fn from(value: String) -> Self {
        SystemValue::String(value)
    }
}

impl From<bool> for SystemValue {
    fn from(value: bool) -> Self {
        SystemValue::Bool(value)
    }
}
impl From<&str> for SystemValue {
    fn from(value: &str) -> Self {
        SystemValue::String(value.to_string())
    }
}
