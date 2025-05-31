#[derive(PartialEq, Debug)]
pub enum NodeMessage {
    HasQuery { location: usize, value: u32 },
    HasResponse(bool),
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
                None => NodeMessage::HasResponse(false),
                Some(val) => {
                    if *val == value {
                        NodeMessage::HasResponse(true)
                    } else {
                        NodeMessage::HasResponse(false)
                    }
                }
            },
            NodeMessage::HasResponse(has_it) => NodeMessage::End,
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
    assert_eq!(response1, NodeMessage::HasResponse(true));

    let response2 = node.receive(NodeMessage::HasQuery {
        location: 1,
        value: 1,
    });
    assert_eq!(response2, NodeMessage::HasResponse(false));
}
