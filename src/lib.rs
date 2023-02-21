use std::rc::Rc;


#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub enum CollatzTree {
    Leaf {
        value: u128,
        odd: Box<CollatzTree>,
        even: Box<CollatzTree>,
    },
    Empty,
}

impl CollatzTree {
    /// Create a new Empty CollatzTree instance
    pub fn new() -> Self {
        CollatzTree::Empty
    }
    /// Create a new CollatzTree instance with initial value
    pub fn create(val: u128) -> Self {
        CollatzTree::Leaf { 
            value: val, 
            odd: Box::new(CollatzTree::Empty), 
            even: Box::new(CollatzTree::Empty) 
        }
    }
    /// Insert a new value in the CollatzTree if it does not already exist in the tree
    ///
    /// If the value already exists the function will return nothing
    pub fn insert(&mut self, new_value: u128) {
        match self {
            CollatzTree::Leaf { 
                ref value, 
                ref mut odd, 
                ref mut even 
            } => {
                if *value == new_value {
                        return;
                }
                match new_value % 2 {
                    0 => even.insert(new_value),
                    1 => odd.insert(new_value),
                    _ => panic!()
                }
            },
            CollatzTree::Empty => {
                *self = CollatzTree::create(new_value);
            }
        }
    }
    /// Print the CollatzTree to the console
    pub fn print(&self, indent: u32) {
        println!("{:?}", self);
        //TODO: make this better
    }
    //TODO: Get function
    //      will return an Option that either holds a reference to a
    //      CollatzTree or None if the value isn't found
    //
    //      This will be used to find the location in the tree to insert
    //      the next value instead of just creating a tree with all odd
    //      and even
    pub fn get(&self, val: u128) -> Option<CollatzTree> {
         match self {
            CollatzTree::Leaf { 
                ref value, 
                ref odd, 
                ref even 
            } => {
                if *value == val {
                    return Some(self.clone());
                }
                if odd.contains(val) {
                    return odd.clone().get(val);
                }
                else {
                    return even.clone().get(val);
                }
            },
            CollatzTree::Empty => {
                return None;
            }
        }
    }

    //TODO: Contains function
    //      will return a boolean that indicates whether the CollatzTree
    //      contains the value specified or not
    pub fn contains(&self, val: u128) -> bool {
        match self {
            CollatzTree::Leaf { 
                ref value, 
                ref odd, 
                ref even 
            } => {
                if *value == val {
                    return true;
                }
                return odd.contains(val) || even.contains(val);
            },
            CollatzTree::Empty => {
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CollatzTree;

    #[test]
    fn new_tree_1() {
        let tree = CollatzTree::new();

        assert_eq!(tree, CollatzTree::Empty);
    }

    #[test]
    fn new_tree_2() {
        let tree = CollatzTree::new();

        assert_ne!(tree, CollatzTree::Leaf { value: 2, odd: Box::new(CollatzTree::Empty), even: Box::new(CollatzTree::Empty)});
    }

    #[test]
    fn create_tree_1() {
        let tree = CollatzTree::create(1);

        assert_eq!(tree, CollatzTree::Leaf { value: 1, odd: Box::new(CollatzTree::Empty), even: Box::new(CollatzTree::Empty)});
    }

    #[test]
    fn create_tree_2() {
        let tree = CollatzTree::create(3);

        assert_ne!(tree, CollatzTree::Leaf { value: 1, odd: Box::new(CollatzTree::Empty), even: Box::new(CollatzTree::Empty)});
    }

    #[test]
    fn insert_tree_1() {
        let mut tree = CollatzTree::create(1);
        tree.insert(2);

        assert_eq!(tree, CollatzTree::Leaf { value: 1, 
            odd: Box::new(CollatzTree::Empty), 
            even: Box::new(CollatzTree::Leaf { 
                value: 2, 
                odd: Box::new(CollatzTree::Empty), 
                even: Box::new(CollatzTree::Empty) }) 
            });
    }

    #[test]
    fn insert_tree_2() {
        let mut tree = CollatzTree::create(1);
        tree.insert(3);

        assert_eq!(tree, CollatzTree::Leaf { value: 1, 
            even: Box::new(CollatzTree::Empty), 
            odd: Box::new(CollatzTree::Leaf { 
                value: 3, 
                odd: Box::new(CollatzTree::Empty), 
                even: Box::new(CollatzTree::Empty) }) 
            });
    }
    
    #[test]
    fn insert_tree_3() {
        let mut tree = CollatzTree::create(1);
        tree.insert(1);

        assert_eq!(tree, CollatzTree::Leaf { value: 1, 
            even: Box::new(CollatzTree::Empty), 
            odd: Box::new(CollatzTree::Empty) 
            });
    }

    #[test]
    fn contains_tree_1() {
        let tree = CollatzTree::create(1);
        
        assert!(tree.contains(1));
    }

     #[test]
    fn contains_tree_2() {
        let mut tree = CollatzTree::create(1);
        tree.insert(2);
        tree.insert(3);

        assert!(tree.contains(2));
        assert!(tree.contains(3));
    }

    #[test]
    fn get_tree_1() {
        let tree = CollatzTree::create(1);

        assert_eq!(tree.get(1).unwrap(), CollatzTree::Leaf { value: 1, odd: Box::new(CollatzTree::Empty), even: Box::new(CollatzTree::Empty) });
    }

    #[test]
    fn get_tree_2() {
        let mut tree = CollatzTree::create(1);
        tree.insert(2);

        assert_eq!(tree.get(2).unwrap(), CollatzTree::Leaf { value: 2, odd: Box::new(CollatzTree::Empty), even: Box::new(CollatzTree::Empty) });
    }

    #[test]
    fn get_tree_3() {
        let mut tree = CollatzTree::create(2);
        tree.insert(3);
        tree = tree.get(3).unwrap();
        tree.insert(4);

        assert_eq!(tree.get(3).unwrap(), CollatzTree::Leaf { value: 3,
            odd: Box::new(CollatzTree::Empty),
            even: Box::new(CollatzTree::Leaf { value: 4,
                odd: Box::new(CollatzTree::Empty),
                even: Box::new(CollatzTree::Empty) })
            });
    }
}
