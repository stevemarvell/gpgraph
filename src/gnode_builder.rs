use crate::gnode::{GNode, GNodeId};
use autoincrement::AutoIncrement;

pub struct GNodeBuilder {
    id_generator: AutoIncrement<GNodeId>,
    number1: i64,
    number2: i64,
}

impl GNodeBuilder {
    pub fn new() -> GNodeBuilder {
        GNodeBuilder {
            id_generator: GNodeId::init(),
            number1: 0,
            number2: 0,
        }
    }

    pub fn number1(&mut self, number1: i64) -> &mut Self {
        self.number1 = number1;
        self
    }

    pub fn number2(&mut self, number2: i64) -> &mut Self {
        self.number2 = number2;
        self
    }

    pub fn build(&mut self) -> GNode {
        GNode::new(self.id_generator.pull(), self.number1, self.number2)
    }
}
