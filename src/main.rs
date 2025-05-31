mod node;

use node::{NodeMessage, NodeState};

fn main() {
    println!("Hello, world!");
}

#[allow(unused)]
fn protocol(node1: &mut NodeState, node2: &mut NodeState) {
    let mut message = node1.start();
    while message != NodeMessage::End {
        let response = node2.receive(message);
        message = node1.receive(response);
    }
}

#[test]
fn basic_protocol () {
    let data = vec![1];
    let mut node1 = NodeState::new(&data);
    let mut node2 = NodeState::new(&data);

    protocol(&mut node1, &mut node2);

    assert_eq!(node1.common, vec![1]);
    assert_eq!(node2.common, vec![1]);
}

fn basic_protocol_partial () {
    let data = vec![7,9];
    let data2 = vec![8,9];
    let mut node1 = NodeState::new(&data);
    let mut node2 = NodeState::new(&data2);

    protocol(&mut node1, &mut node2);

    assert_eq!(node1.common, vec![9]);
    assert_eq!(node2.common, vec![9]);
}