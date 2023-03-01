use std::fmt;
use autoincrement::prelude::*;

#[derive(Incremental, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord, Debug)]
pub struct GNodeId(u32);

#[derive(Copy, Clone, Debug)]
pub struct GNode {
    pub id: GNodeId,
    pub number1: i64,
    pub number2: i64,
}

impl GNode {
    pub fn new(id: GNodeId, number1: i64, number2: i64) -> GNode {
        GNode {
            id,
            number1,
            number2,
        }
    }
}
impl fmt::Display for GNodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Display for GNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GNode {{ id: {}, number1: {}, number2: {} }}", self.id, self.number1, self.number2)
    }
}