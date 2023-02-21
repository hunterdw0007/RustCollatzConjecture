use std::collections::HashSet;

use collatz::CollatzTree;

fn main() {
    println!("Following the numbers in the Collatz Conjecture:");

    let mut tree = CollatzTree::new();

    for i in 0..10 {
        tree.insert(i);
    }

    CollatzTree::print(&tree, 0);
}
