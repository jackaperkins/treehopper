#[derive(PartialEq, Debug)]
pub enum NodeMessage {
    HasQuery { location: usize, value: u32 },
    HasResponse{ location: usize, has: bool},
    End,
}

pub struct NodeState<'a> {
    data: &'a Vec<u32>, // set we're testing the other node for
    data_pointer: u32,
    common: Vec<u32>,
}

impl<'a> NodeState<'a> {
    pub fn new(data: &'a Vec<u32>) -> Self {
        NodeState {
            data,
            common: vec![],
            data_pointer: 0,
        }
    }

    pub fn start() -> NodeMessage {
        NodeMessage::End
    }

    pub fn receive(&mut self, message: NodeMessage) -> NodeMessage {
        match message {
            NodeMessage::HasQuery { location, value } => match self.data.get(location) {
                None => NodeMessage::End,
                Some(val) => {
                    if *val == value {
                        self.common.push(value);
                        NodeMessage::HasResponse{location, has: true}
                    } else {
                        NodeMessage::HasResponse{location, has: false}
                    }
                }
            },
            NodeMessage::HasResponse{location, has} => NodeMessage::End,
            NodeMessage::End => NodeMessage::End,
        }
    }
}

fn protocol() {}

#[test]
fn basic_has_query() {
    let data = vec![1, 2, 3];
    let mut node = NodeState::new(&data);
    let response1 = node.receive(NodeMessage::HasQuery {
        location: 0,
        value: 1,
    });
    assert_eq!(response1, NodeMessage::HasResponse{location: 0, has:true});

    let response2 = node.receive(NodeMessage::HasQuery {
        location: 1,
        value: 1,
    });
    assert_eq!(response2, NodeMessage::HasResponse{location: 1, has:false});

    let response3 = node.receive(NodeMessage::HasQuery {
        location: 9,
        value: 1,
    });
    // expect end when either side runs out of elements
    assert_eq!(response3, NodeMessage::End);
}
