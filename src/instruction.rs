use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    PushConstant (u16),
    PushTrue,
    PushFalse,
    PushNull,

    Negate,
    ToNumber,
    Not,
    Add,
    Subtract,
    Multiply,
    Divide,

    Equal,
    NotEqual,
    GreaterEqual,
    LessEqual,
    Greater,
    Less,

    Trace,

    PushVariable(u16),
    Assign(u16),
    DeclareAssign(u16),
    DeclareAssignConstant(u16, u16), //assignid, constid
    FnCall(u16),
    Closure(u16, u16), //assignid, constid

    JumpIfFalsy(u16),
    PopAndJumpIfFalsy(u16), //always pop, that is
    JumpIfTruthy(u16),
    Jump(u16),
    JumpPlaceholder,
    Pop, Return,

    ReservePlaceholder,
    Reserve(u16)
}