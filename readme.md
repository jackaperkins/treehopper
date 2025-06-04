# Treehopper
Abstract protocol for comparing sets. My _naive attempt_ without much research, as a learning exercise. Assume data is single vector of u32 for now, worry about tree traversal later.

## Drafts
- [simple.rs](src/simple.rs) - Simple protocol where two nodes send each position-wise data, each ending up with the shared state. Linear time because it really sucks
- [challenge.rs](src/challenge.rs) - Salted hash protocol where responding node set on an initial salt to hash their data with, and then issue new salt-based challenges back to double check. Poor man's diffie-hellman, we're not sorting anything so every search is O(n)
- range.rs - TODO range based set reconciliation

## Todo
- [x] Mock in very basics of two nodes that can exchange messages
- [x] Test where we ask for a single element comparison
- [x] Test full protocol runthrough with various sets of datas
- [x] think about various failure cases, make sure both nodes end up in a end state when either of them fails