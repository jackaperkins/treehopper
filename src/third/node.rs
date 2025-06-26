//

use std::{hash::Hash, marker::PhantomData};

use sha2::Digest;

pub type SessionSalt = [u8; 32];
pub type RoleSalt = [u8; 1];

pub enum NodeRole {
    Leader,
    Follower,
}

impl NodeRole {
    fn salt(&self) -> RoleSalt {
        match self {
            NodeRole::Leader => [0],
            NodeRole::Follower => [1],
        }
    }
}

pub struct Node<T> {
    session_salt: SessionSalt,
    role: NodeRole,
    _marker: PhantomData<T>,
}

impl<T> Node<T>
where
    T: Hash,
{
    pub fn new(session_salt: SessionSalt, role: NodeRole) -> Node<T> {
        Node {
            session_salt,
            role,
            _marker: PhantomData,
        }
    }

    pub fn start(&self) -> Result<Message<T>, ApiError> {
        if matches!(self.role, NodeRole::Follower) {
            return Err(ApiError::FollowerCannotStart);
        }
        Err(())
    }

    pub fn receive(message: Message<T>) -> Message<T> {
        Message::Done
    }

    pub fn final_results() {}
}

enum Message<T>
where
    T: Hash,
{
    Challenge(T),
    Reponse(Option<T>),
    Fail(ProtocolError),
    Done,
}

enum ProtocolError {
    UnexpectedMessage,
    VerificationFailed,
}

enum ApiError {
    FollowerCannotStart,
}
