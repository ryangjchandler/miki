use std::cmp::{PartialOrd, Ordering};
use std::fmt::{Display};
use std::ops::{Sub, Add};

#[derive(PartialEq, Clone)]
enum Object {
    String(String),
    Int(i64),
    Bool(bool),
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a.partial_cmp(b),
            _ => unreachable!("Unimplemented partial_cmp handling.")
        }
    }
}

impl Sub for Object {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Self::Int(a - b),
            _ => unreachable!("Unimplemeted sub operation.")
        }
    }
}

impl Add for Object {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Self::Int(a + b),
            _ => unreachable!("Unimplemeted add operation.")
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Int(n) => n.to_string(),
            _ => unreachable!("Unimplemented display logic.")
        })
    }
}