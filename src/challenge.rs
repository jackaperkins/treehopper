/// Naive attempt #2, now a more complex multi-stage protocol with proper hash challenge to build our common set

/// After being queried about particular element, repond with a proof using a new hash + it's salt
pub struct ChallengeReponsePair {
  salt: u32,
  hash: u32
}

// a is the initiator, b is the responder.
pub enum NodeMessage {
  Start, // a starts
  Ready {salt: u32}, // b agrees and chooses a salt for the initial state
  ChallengeQuery {hash: u64} , // a queries with the salted hash of a particular value
  ChallengeReponse (Option<ChallengeReponsePair>), // response of either None or Some with new proof for a that b has the unhashed value
  End // a or b should be able to hang up anytime
}


pub enum NodeState {
  Initial,
  Ready, // with first hashed state
  AwaitReponse
}


pub struct Node<'a> {
  data: &'a Vec<u32>,
  state: NodeState
}

impl <'a> Node<'a> {
  fn new(data: &'a Vec<u32>) -> Node<'a> {
    Node {
      data,
      state: NodeState::Initial
    }
  }

  fn recieve_message(&mut self, message: NodeMessage) -> NodeMessage {
    match self.state {
      NodeState::Initial => {
        match message {
          NodeMessage::Ready { salt } => {

          },
          _ => {
            return NodeMessage::End;
          }
        }
      },
      NodeState::AwaitReponse => {

      },
      _ => {}
    }
    NodeMessage::End
  }
}


#[test] 
fn initialization () {
  let data = vec![1,2,3];
  let n = Node::new(&data);

}