# Treehopper
Abstract protocol for comparing sets. My naive attempt without much research, as a learning exercise. Assume data is single vector of u32 for now, worry about tree traversal later.

- [x] Mock in very basics of two nodes that can exchange messages
- [x] Test where we ask for a single element comparison
- [ ] Test full protocol runthrough with various sets of datas
- [ ] think about various failure cases, make sure both nodes end up in a end state when either of them fails
- [ ] swap in a tree?