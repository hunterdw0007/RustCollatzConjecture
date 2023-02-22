use collatz::CollatzTree;

fn main() {
    println!("Following the numbers in the Collatz Conjecture:");

    let mut tree = CollatzTree::create(1);

    let mut numbers: Vec<u128>;

    for i in 1..11 {
        numbers = collatz(i);

    }

    CollatzTree::print(&tree, 0);
}

pub fn collatz(start: u128) -> Vec<u128> {
    let mut value = start;
    let mut output = vec![start];

    while value != 1 {
        
        if value % 2 == 0 {
            value = value / 2;
        }
        else {
            value = 3 * value + 1;
        }
        
        output.push(value);
    }
    output
}

pub fn insert_at_prev(tree: &mut CollatzTree, numbers: &mut Vec<u128>) {
    let mut prev = numbers.pop();

    for x in numbers {
        unimplemented!();
        //get a tree with its root as prev
        //insert x to that tree
    }
}
