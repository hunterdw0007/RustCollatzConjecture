
#[derive(Debug)]
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
    pub fn new() -> Self {
        CollatzTree::Empty
    }

    pub fn create(val: u128) -> Self {
        CollatzTree::Leaf { 
            value: val, 
            odd: Box::new(CollatzTree::Empty), 
            even: Box::new(CollatzTree::Empty) 
        }
    }
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
}
