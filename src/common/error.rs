use std::{error, fmt};

use x86asm::instruction::{
    mnemonic::Mnemonic,
    operand::{register::Register, Operand},
};

use crate::frontend::lexer::token::TokenKind;

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedChar {
        actual: char,
    },
    UnexpectedToken {
        expected: Option<TokenKind>,
        actual: TokenKind,
    },
    ExpectedInteger {
        actual: TokenKind,
    },
    ExpectedIdent {
        actual: TokenKind,
    },
    UnknownPseudoOp {
        name: String,
    },

    UnexpectedMnemonic {
        actual: Mnemonic,
    },
    UnexpectedOperand {
        actual: Operand,
    },
    UnexpectedRegister {
        actual: Register,
    },
    MismatchOperand {
        left: Operand,
        right: Operand,
    },
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        match self {
            UnexpectedChar { actual } => write!(f, "unexpected char: '{}'", actual),
            UnexpectedToken { expected, actual } => {
                write!(f, "unexpected {:?}", actual)?;
                if let Some(expected) = expected {
                    write!(f, ", expecting {:?}", expected)?;
                }
                Ok(())
            }
            ExpectedInteger { actual } => write!(f, "expected integer, but got {:?}", actual),
            ExpectedIdent { actual } => write!(f, "expected identifier, but got {:?}", actual),
            UnknownPseudoOp { name } => write!(f, "unknown pseudo-op: '{}'", name),

            UnexpectedMnemonic { actual } => write!(f, "unexpected {:?}", actual),
            UnexpectedOperand { actual } => write!(f, "unexpected {:?}", actual),
            UnexpectedRegister { actual } => write!(f, "unexpected {:?}", actual),
            MismatchOperand { left, right } => {
                write!(f, "operand type mismatch {:?} and {:?}", left, right)
            }
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl error::Error for Error {}
