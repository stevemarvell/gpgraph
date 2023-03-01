mod gnode;
mod gnode_builder;

use gnode_builder::GNodeBuilder;

fn main() {
    let mut builder = GNodeBuilder::new();
    let node1 = builder.number1(123).number2(456).build();
    let node2 = builder.number1(789).number2(101112).build();
    let node3 = builder.number1(131415).number2(161718).build();

    println!("{}", node1);
    println!("{}", node2);
    println!("{}", node3);
}




