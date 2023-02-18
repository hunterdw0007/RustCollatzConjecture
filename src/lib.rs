#[derive(Debug)]
struct BinaryTree {
    pub value: u128,
    pub first_child: Option<Box<BinaryTree>>,
    pub second_child: Option<Box<BinaryTree>>,
}

impl BinaryTree {
    pub fn find(self, val:&u128) -> Option<BinaryTree> {
        if self.value == *val {
            return Some(self);
        }
        else {
            let mut found : Option<BinaryTree> = self.first_child.unwrap().find(val);
            if found.is_none() {
                found = self.second_child.unwrap().find(val);
            }
            return found;
        }
    }
}
