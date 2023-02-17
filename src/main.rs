use std::collections::HashSet;

fn main() {
    println!("Following the numbers in the Collatz Conjecture:");

    let mut numbers = HashSet::new();

    numbers.insert(1);

    let mut next_unreached : u128 = 2;

    while next_unreached < 10000 {         
        let mut reached = Vec::new();
        reached.push(next_unreached);
        let mut current = next_unreached;
        next_unreached += 1;
        let mut found = false;

        while !found {
            if numbers.contains(&current) {
                println!("{:?}", reached);
                for x in &reached {
                    numbers.insert(*x);
                }
                found = true;
            }

            if current % 2 == 0 {
                current = current / 2;
                reached.push(current);
            }
            else {
                current = current * 3 + 1;
                reached.push(current);
            }
        }
    }
    println!("{:?}", numbers);
}
