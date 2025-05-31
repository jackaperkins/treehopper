# Treehopper
Abstract protocol for comparing sets. My naive attempt without much research, as a learning exercise. Assume data is single vector of u32 for now, worry about tree traversal later.

## Basic idea
check common elements position-wise between two arrays, wrapped in NodeStates. Return a counter of how many iterations it took. NodeStates feed each other NodeMessages to communicate state, with one being the initator sending queries and the other 
 
For example `[1,2,3]` and `[1,2,3]` have common elements `[1,2,3]`
 
But `[1,2,3]` and `[3,2,1]` only have `[2]` in common
 
This is only metaphorically related to walking a tree, this code serves no useful purpose.

## Todo
- [x] Mock in very basics of two nodes that can exchange messages
- [x] Test where we ask for a single element comparison
- [x] Test full protocol runthrough with various sets of datas
- [x] think about various failure cases, make sure both nodes end up in a end state when either of them fails
- [ ] swap in a tree?