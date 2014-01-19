use primitive::{RustFunc, BoundFn};

#[deriving(Clone, Eq)]
pub enum Element {
    Symbol(~str),
    Number(i64),
    String(~str),
    Character(char),
    ParseError(~str),
    EvalError(~str),
    List(~[Element]),
    Vec(~[Element]),
    Function(~BoundFn),
    FuncPrimitive(~RustFunc),
    nil
}

