mod node;

use node::{NodeMessage, NodeState};

fn main() {
    println!("Hello, world!");
}


/// check common elements position-wise between two arrays, wrapped in NodeStates. Return a counter of how many iterations it took
/// 
/// For example `[1,2,3]` and `[1,2,3]` have common elements `[1,2,3]`
/// 
/// But `[1,2,3]` and `[3,2,1]` only have `[2]` in common
/// 
/// This is only metaphorically related to walking a tree, this code serves no useful purpose now
#[allow(unused)]
fn protocol(node1: &mut NodeState, node2: &mut NodeState) -> usize {
    let mut message = node1.start();
    let mut counter = 0;
    loop {
        counter += 1;
        let response = node2.receive(message);
        if message == NodeMessage::End {
            break;
        }
        message = node1.receive(response);
    }
    counter
}

#[test]
fn basic_protocol() {
    let data = vec![1];
    let mut node1 = NodeState::new(&data);
    let mut node2 = NodeState::new(&data);

    protocol(&mut node1, &mut node2);

    assert_eq!(node1.common, vec![1]);
    assert_eq!(node2.common, vec![1]);
}

#[test]
fn basic_protocol_partial() {
    let data = vec![7, 9];
    let data2 = vec![8, 9];
    let mut node1 = NodeState::new(&data);
    let mut node2 = NodeState::new(&data2);

    protocol(&mut node1, &mut node2);

    assert_eq!(node1.common, vec![9]);
    assert_eq!(node2.common, vec![9]);
}

#[test]
fn protocol_odd_lengths() {
    let data = vec![7, 9, 10, 11, 12, 13, 14];
    let data2 = vec![8, 9];
    let mut node1 = NodeState::new(&data);
    let mut node2 = NodeState::new(&data2);

    let iterations = protocol(&mut node1, &mut node2);

    assert_eq!(node1.common, vec![9]);
    assert_eq!(node2.common, vec![9]);

    // prove that we dont spin through elements once the other side hung up
    assert_eq!(iterations, 4);
}

#[test]
fn protocol_odd_lengths_reversed() {
    let data = vec![8, 9];
    let data2 = vec![7, 9, 10, 11, 12, 13, 14];
    let mut node1 = NodeState::new(&data);
    let mut node2 = NodeState::new(&data2);

    let iterations = protocol(&mut node1, &mut node2);

    assert_eq!(node1.common, vec![9]);
    assert_eq!(node2.common, vec![9]);

    // initiator hangs up quicker because it's shorter, so only 2 loops
    assert_eq!(iterations, 3);
}
