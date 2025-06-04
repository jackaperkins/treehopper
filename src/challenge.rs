//! Naive attempt #2, now a more complex multi-stage protocol with a salted hash challenge.
//!
//! Construct a node given the secret `data` and whether it is the protocol `Leader` (a) or `Follower` (b)
//!
//! Nodes communicate by sending each other `NodeMessage`
//!
//! ## The Protocol
//! - a sends b `Start`
//! - b generates a salt value, hashes its data and shares the salt with a via `Initialize`
//! - a hashes its data with the same salt value
//! - a iterates each data, sending its hashed value to b via `ChallengeQuery`
//!   - if b doesn't have the matching hashed data it knows it's not in the set
//!     - b returns `ChallengeReponse(None)`
//!   - if b has the same value, it knows they share the same element and makes a note
//!     - b returns `ChallengeReponse(Some(new salt, new hashed data))` as a challenge for a
//!     - a computes its own local version of (data + new salt) to check if b really has original data, making note
//!       - in the case that a cannot derive the same new hash, it sends `Fail` and should quit
//! - a sends `Done` when it runs out of elements
//! - in the case that either side recieves `Done` or `Fail` it should close the network connection and finish
use std::collections::HashSet;

use rand::{Rng, distr::Alphanumeric};

#[derive(PartialEq, Debug, Clone)]
pub struct ChallengeReponsePair {
    pub salt: String,
    pub hash: String,
}

// a is the initiator, b is the responder.
#[derive(PartialEq, Debug, Clone)]
pub enum NodeMessage {
    Start,                                          // a starts
    Initialize { salt: String }, // b agrees and chooses a salt for the initial state
    ChallengeQuery { hash: String }, // a queries with the salted hash of a particular value
    ChallengeReponse(Option<ChallengeReponsePair>), // response of either None or Some with new proof for a that b has the unhashed value
    Fail { reason: String },                        //
    Done,                                           // a or b should be able to hang up anytime
}

// simple state machine
#[allow(unused)]
pub enum NodeType {
    Leader,
    Follower,
}

pub struct Node<'a> {
    node_type: NodeType,
    data: &'a [String],
    data_index: usize,
    first_challenge: bool,
    salt: Option<String>,
    data_hashed: Vec<String>,
    /// data we have in common with the peer
    data_common: HashSet<String>,
}

impl<'a> Node<'a> {
    pub fn new(data: &[String], node_type: NodeType) -> Node {
        Node {
            node_type,
            data,
            data_index: 0,
            first_challenge: true,
            salt: None,
            data_hashed: vec![],
            data_common: HashSet::new(),
        }
    }

    pub fn start(&mut self) -> NodeMessage {
        match self.node_type {
            NodeType::Leader => NodeMessage::Start,
            NodeType::Follower => NodeMessage::Fail {
                reason: "Cannot call start on follower node".to_string(),
            },
        }
    }

    pub fn recieve_message(&mut self, message: NodeMessage) -> NodeMessage {
        match self.node_type {
            NodeType::Leader => match message {
                NodeMessage::Initialize { salt } => match self.salt {
                    Some(_) => {
                        return NodeMessage::Fail {
                            reason: "Node recieved initialize when already initialized".to_string(),
                        };
                    }
                    None => {
                        self.salt = Some(salt);
                        self.hash_data();
                        self.next_challenge()
                    }
                },
                NodeMessage::ChallengeReponse(response) => match response {
                    None => self.next_challenge(),
                    Some(response2) => match self.data.get(self.data_index) {
                        Some(original_data) => {
                            let new_hash = hash_value(&original_data, &response2.salt);
                            if new_hash == response2.hash {
                                self.data_common.insert(original_data.clone());
                            }
                            self.next_challenge()
                        }
                        None => NodeMessage::Fail {
                            reason: format!("Protocol responder gave bad new salt challenge for current data")
                        },
                    },
                },
                NodeMessage::Done => NodeMessage::Done,
                NodeMessage::Fail { reason } => NodeMessage::Fail {
                    reason: format!("Protocol responder failed: {}", reason)
                },
                _ => {
                    return NodeMessage::Fail {
                        reason: format!("Unsupported message for this node state: {:?}", message),
                    };
                }
            },
            NodeType::Follower => match message {
                NodeMessage::Start => {
                    if self.salt.is_some() {
                        return NodeMessage::Fail {
                            reason: "Recieved start when already initialized".to_string(),
                        };
                    }
                    let salt = generate_salt();
                    self.salt = Some(salt.clone());
                    self.hash_data();
                    NodeMessage::Initialize { salt: salt }
                }
                NodeMessage::Initialize { salt: _ } => {
                    return NodeMessage::Fail {
                        reason: "Node recieved initialize when already initialized".to_string(),
                    };
                }
                NodeMessage::ChallengeQuery { hash } => {
                    let found = self.data_hashed.iter().position(|h| *h == hash);
                    match found {
                        None => NodeMessage::ChallengeReponse(None),
                        Some(index) => {
                            let original_data = self.data[index].clone();
                            let new_salt = generate_salt();
                            let new_hash = hash_value(&original_data, &new_salt);
                            self.data_common.insert(original_data);
                            NodeMessage::ChallengeReponse(Some(ChallengeReponsePair {
                                salt: new_salt,
                                hash: new_hash,
                            }))
                        }
                    }
                }
                NodeMessage::Done => NodeMessage::Done,
                NodeMessage::Fail { reason: _ } => NodeMessage::Fail {
                    reason: "Protocol leader failed".to_owned(),
                },
                NodeMessage::ChallengeReponse(_) => NodeMessage::Fail {
                    reason: "Received challenge response".to_owned(),
                },
            },
        }
    }

    fn next_challenge(&mut self) -> NodeMessage {
        if self.first_challenge {
            self.first_challenge = false;
        } else {
            self.data_index += 1;
        }
        match self.data_hashed.get(self.data_index) {
            None => NodeMessage::Done,
            Some(next_hashed_data) => {
                let message = NodeMessage::ChallengeQuery {
                    hash: next_hashed_data.clone(),
                };
                return message;
            }
        }
    }

    /// hash our local data array
    fn hash_data(&mut self) {
        match &self.salt {
            None => {}
            Some(salt) => {
                self.data_hashed = self.data.iter().map(|val| hash_value(val, &salt)).collect();
            }
        }
    }
}

#[allow(unused)]
fn generate_salt() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

/// fake function hashing something, replace with real hashing later
#[allow(unused)]
fn hash_value(value: &String, salt: &String) -> String {
    value.to_owned() + salt
}

/// Phony protocol, in reality we'd only have one side of this but the logic would be the same.
/// In the case that we're the follower we'd know because we'd start by receiving the `Start` message
/// In both cases we'd close our connection once we recieved Done or Fail
#[allow(unused)]
fn protocol(leader: &mut Node, follower: &mut Node) {
    let mut message = leader.start();
    loop {
        println!("Message from leader: {:?}", message);
        let reply = follower.recieve_message(message.clone());
        println!("Message from follower: {:?}", reply);
        if matches!(&message, NodeMessage::Fail { reason: _ }) {
            break;
        }
        if matches!(&message, NodeMessage::Done) {
            break;
        }
        message = leader.recieve_message(reply);
    }
}


// tests

#[test]
fn initialization() {
    let data = fix_array(vec!["a", "b", "c"]);
    let mut n = Node::new(&data, NodeType::Follower);

    let response = n.recieve_message(NodeMessage::Start);
    let result = match response {
        NodeMessage::Initialize { salt: _ } => true,
        _ => false,
    };
    assert!(result);
}

#[test]
fn protocol_basics() {
    let data = fix_array(vec!["1", "b", "c"]);
    let mut n1 = Node::new(&data, NodeType::Leader);
    let mut n2 = Node::new(&data, NodeType::Follower);

    protocol(&mut n1, &mut n2);

    let data_set =  HashSet::from_iter(data.iter().map(String::from));
    assert_eq!(n1.data_common, data_set);
    assert_eq!(n2.data_common, data_set);
}

#[test]
fn protocol_order() {
    let data = fix_array(vec!["1", "b", "c"]);
    let data2 = fix_array(vec!["b", "c", "1"]);
    let mut n1 = Node::new(&data, NodeType::Leader);
    let mut n2 = Node::new(&data2, NodeType::Follower);

    protocol(&mut n1, &mut n2);

    let data_set =  HashSet::from_iter(data.iter().map(String::from));
    assert_eq!(n1.data_common, data_set);
    assert_eq!(n2.data_common, data_set);
}

#[test]
fn protocol_no_common() {
    let data = fix_array(vec!["1", "2", "3"]);
    let data2 = fix_array(vec!["a", "b", "c"]);
    let mut n1 = Node::new(&data, NodeType::Leader);
    let mut n2 = Node::new(&data2, NodeType::Follower);

    protocol(&mut n1, &mut n2);

    assert_eq!(n1.data_common.len(), 0);
    assert_eq!(n2.data_common.len(), 0);
}


// TODO add further tests with 'evil' peers who send messages at wrong times?

#[allow(unused)]
fn fix_array(input: Vec<&str>) -> Vec<String> {
    input.into_iter().map(String::from).collect::<Vec<String>>()
}
