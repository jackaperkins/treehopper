#[derive(PartialEq, Debug)]
pub enum NodeMessage {
    HasQuery { location: usize, value: u32 },
    HasResponse { location: usize, has: bool },
    End,
}

pub struct NodeState<'a> {
    data: &'a Vec<u32>, // set we're testing the other node for
    index: usize,
    common: Vec<u32>,
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

    assert_eq!(message, NodeMessage::HasQuery { location: 0, value: 1 });
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

    node.receive(NodeMessage::HasResponse { location: 0, has: true });
    node.receive(NodeMessage::HasResponse { location: 2, has: true });

    assert_eq!(node.common, vec![1,3]);
}

#[test]
fn common_state_for_responder() {
    let data = vec![1, 2, 3];
    let mut node = NodeState::new(&data);

    node.receive(NodeMessage::HasQuery { location: 0, value: 1 });
    node.receive(NodeMessage::HasQuery { location: 2, value: 3 });

    assert_eq!(node.common, vec![1,3]);
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
