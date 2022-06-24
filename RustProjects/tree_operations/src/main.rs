#[derive(Debug)]
enum Node {
    // Single node requires pointers for both left & right
    ChildNode(i32, Box<Node>, Box<Node>),
    LeafNode,
}

impl Node {
    fn add(&mut self, &val: &i32) {
        // add something here
        
    }
}

fn main() {
    let binary_tree = Node::LeafNode;
    print!("{:?}", binary_tree);
}

/* Common operations of all kinds of trees
# Enumerating all the items
# Enumerating a section of a tree
# Searching for an item
# Adding a new item at a certain position on the tree
# Deleting an item
# Pruning: Removing a whole section of a tree
# Grafting: Adding a whole section to a tree
# Finding the root for any node
# Finding the lowest common ancestor of two nodes
*/

/* Structure of the binary tree (or a single node)
# n-times Nested
# Data to implement, Left-pointer, Right-pointer
*/

