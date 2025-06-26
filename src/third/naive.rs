use std::hash::Hash;

use crate::third::SessionSalt;
use crate::third::traits::PrivateSession;

struct NaiveSession<'a, T>
where
    T: Hash,
{
    data: &'a [T],
    index: usize,
}

type HashDigest = [u8; 32];

impl<'a, T: Hash> PrivateSession<HashDigest> for NaiveSession<'a, T> {
    fn next_challenge(&mut self, session_salt: SessionSalt) -> Option<HashDigest> {
        None
    }

    fn respond_to_challenge(
        &mut self,
        session_salt: SessionSalt,
        challenge: HashDigest,
    ) -> Option<HashDigest> {
        None
    }

    fn verify_challenge(&self, session_salt: SessionSalt, challenge: HashDigest) -> bool {
        false
    }
}
