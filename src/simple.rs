/// Naive attempt #1, thinking about the basics of protocols in rust
/// We check common elements position-wise between two arrays, wrapped in NodeStates. Return a counter of how many iterations it took
/// 
/// For example `[1,2,3]` and `[1,2,3]` have common elements `[1,2,3]`
/// 
/// But `[1,2,3]` and `[3,2,1]` only have `[2]` in common
/// 
/// This is only metaphorically related to walking a tree, this code serves no useful purpose now

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NodeMessage {
    HasQuery { location: usize, value: u32 },
    HasResponse { location: usize, has: bool },
    End,
}

pub struct NodeState<'a> {
    data: &'a Vec<u32>, // set we're testing the other node for
    index: usize,
    pub common: Vec<u32>,
}

impl<'a> NodeState<'a> {
    pub fn new(data: &'a Vec<u32>) -> Self {
        NodeState {
            data,
            common: vec![],
            index: 0,
        }
    }

    /// Call on the intiator node to get first message
    pub fn start(&mut self) -> NodeMessage {
        self.next_query()
    }

    /// feed messages from other peer in here
    pub fn receive(&mut self, message: NodeMessage) -> NodeMessage {
        match message {
            NodeMessage::HasQuery { location, value } => match self.data.get(location) {
                None => NodeMessage::End,
                Some(val) => {
                    if *val == value {
                        self.common.push(value);
                        NodeMessage::HasResponse {
                            location,
                            has: true,
                        }
                    } else {
                        NodeMessage::HasResponse {
                            location,
                            has: false,
                        }
                    }
                }
            },
            NodeMessage::HasResponse { location, has } => match (location, has) {
                (location, true) => match self.data.get(location) {
                    None => NodeMessage::End,
                    Some(value) => {
                        self.common.push(*value);
                        self.next_query()
                    }
                },
                (_location, false) => self.next_query(),
            },
            NodeMessage::End => NodeMessage::End,
        }
    }

    fn next_query(&mut self) -> NodeMessage {
        let response = if self.index < self.data.len() {
            NodeMessage::HasQuery {
                location: self.index,
                value: self.data[self.index],
            }
        } else {
            NodeMessage::End
        };
        self.index += 1;

        response
    }
}

// tests

#[test]
fn start_node() {
    let data = vec![1, 2, 3];
    let mut node = NodeState::new(&data);

    let message = node.start();

    assert_eq!(
        message,
        NodeMessage::HasQuery {
            location: 0,
            value: 1
        }
    );
}

#[test]
fn basic_has_query() {
    let data = vec![1, 2, 3];
    let mut node = NodeState::new(&data);
    let response1 = node.receive(NodeMessage::HasQuery {
        location: 0,
        value: 1,
    });
    assert_eq!(
        response1,
        NodeMessage::HasResponse {
            location: 0,
            has: true
        }
    );

    let response2 = node.receive(NodeMessage::HasQuery {
        location: 1,
        value: 1,
    });
    assert_eq!(
        response2,
        NodeMessage::HasResponse {
            location: 1,
            has: false
        }
    );

    let response3 = node.receive(NodeMessage::HasQuery {
        location: 9,
        value: 1,
    });
    // expect end when either side runs out of elements
    assert_eq!(response3, NodeMessage::End);
}

#[test]
fn common_state_for_querier() {
    let data = vec![1, 2, 3];
    let mut node = NodeState::new(&data);

    node.receive(NodeMessage::HasResponse {
        location: 0,
        has: true,
    });
    node.receive(NodeMessage::HasResponse {
        location: 2,
        has: true,
    });

    assert_eq!(node.common, vec![1, 3]);
}

#[test]
fn common_state_for_responder() {
    let data = vec![1, 2, 3];
    let mut node = NodeState::new(&data);

    node.receive(NodeMessage::HasQuery {
        location: 0,
        value: 1,
    });
    node.receive(NodeMessage::HasQuery {
        location: 2,
        value: 3,
    });

    assert_eq!(node.common, vec![1, 3]);
}

#[test]
fn bad_response() {
    let data = vec![1, 2, 3];
    let mut node = NodeState::new(&data);

    let response = node.receive(NodeMessage::HasResponse {
        location: 9,
        has: true,
    });

    assert_eq!(response, NodeMessage::End);
}

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
