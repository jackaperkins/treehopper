use crate::third::SessionSalt;

pub trait PrivateSession<T> {
    // Leader makes and sends hash(Secret + Salt I)
    fn next_challenge(&mut self, session_salt: SessionSalt) -> Option<T>;

    // Follower makes and sends hash(Secret + Salt II)
    fn respond_to_challenge(&mut self, session_salt: SessionSalt, challenge: T) -> Option<T>;

    // Leader verifies hash(Secret + Salt II)
    fn verify_challenge(&self, session_salt: SessionSalt, challenge: T) -> bool;
}
