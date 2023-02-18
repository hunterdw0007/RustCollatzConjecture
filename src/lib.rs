#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct NaryTree {
    pub value: u128,
    pub nodes: Vec<NaryTree>,
}

impl NaryTree {
    /// Add new child node to nodes vector
    ///
    pub fn add_child(&mut self, val: u128){
        self.nodes.push(NaryTree { value: val, nodes: Vec::new() });
    }
    
    /// Return an option with a reference to the location where the value is found
    /// Will return None if the value doesn't exist
    pub fn find(&self, val: &u128) -> Option<&NaryTree>{
        if self.nodes.len() == 0 {
            return None;
        }
        if self.value == *val {
            return Some(self);
        }
        else {
            for ele in &self.nodes {
                let found = ele.find(val);
                if found != None {
                    return found;
                }
            }
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::NaryTree;

    #[test]
    fn test_add_child() {
        let mut tree: NaryTree = NaryTree { value: 1, nodes: Vec::new() };
        tree.add_child(2);
        assert_ne!(tree.nodes, Vec::new());
    }

    #[test]
    fn test_not_found() {
        let child: NaryTree = NaryTree { value: 4, nodes: Vec::new() };
        let root: NaryTree = NaryTree { value: 3, nodes: vec![child] };
        assert_eq!(root.find(&2), None);
    }
    #[test]
    fn test_found() {
        let child: NaryTree = NaryTree { value: 4, nodes: Vec::new() };
        let root: NaryTree = NaryTree { value: 3, nodes: vec![child.clone()] };
        assert_eq!(root.find(&4).unwrap(), &child);
    }
}
