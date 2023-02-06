# Treap structure in rust (example)

The main purpose of this code is practicing with pointer in rust.

TREAP data structure is made of a Binary Search TRee + hEAP.

Here also I use a generic (also for practicing), and there are lots
of println! for showing whats going on.

Procedure insert is the only implemented, made of:
 * insert in the Binary Search Tree
 * balance the new inserted node

Consider to use VecDeque in insert phase then pop element from this for balancing make me think that my expectation is to only need parents considered during insertion, and that make me think that maybe is not needed to insert the node as if it weights ∞, but just stop when an higher weight is found.

On other hand it could be my fault, and I can not save the parent pointer for each node (as I want to do).

