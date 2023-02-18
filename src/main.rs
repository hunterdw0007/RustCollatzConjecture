use collatz::NaryTree;

fn main() {
    println!("Following the numbers in the Collatz Conjecture:");

    /*
    * Ideas:
    *
    * With each iteration of the numbers,
    * store the path as a vector
    *
    * After it has gotten to one, reverse the path and begin following the
    * N-ary Tree with each number in the vector
    * When something unreached is found, insert each value as new nodes to the tree
    *
    * Since it's a tree structure, there are no loops
    */ 
    let mut tree = NaryTree{ value: 1, nodes: Vec::new(), };

    let mut reached: Vec<u128> = Vec::new();
    
    for i in 1..11 {         
        collatz(&mut reached, i);
        println!("{:?}", reached);
        reached.clear();
    }
}

pub fn collatz(path: &mut Vec<u128>, n: u128) {
    path.push(n);

    if n == 1 {
        return;
    }

    if n % 2 == 0 {
        collatz(path, n / 2)
    }
    else {
        collatz(path, 3 * n + 1)
    }
}
