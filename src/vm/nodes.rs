pub mod int64;
pub mod bool;
pub mod unit;
pub mod stack;
pub mod closure;
pub mod environment;
pub mod rts_frame;

pub const TAG_LABEL: u8 = 0;
pub const SIZE_LABEL: u8 = 1;
pub const FIRST_CHILD_LABEL: u8 = 2;
pub const LAST_CHILD_LABEL: u8 = 3;

pub enum NodeType {
    Int64,
    Bool,
    Unit,
    Stack,
    Closure,
    Environment,
    RuntimeStackFrame,
}

impl NodeType {
    pub fn get_tag(&self) -> isize {
        match self {
            NodeType::Int64 => -100,
            NodeType::Bool => -101,
            NodeType::Environment => -102,
            NodeType::Closure => -103,
            NodeType::RuntimeStackFrame => -104,
            NodeType::Stack => -105,
            NodeType::Unit => -106,
        }
    }
}