// Problem Statement: https://projecteuler.net/problem=2

use std::collections::HashMap;

struct Fibonacci {
    memo: HashMap<u64, u64>,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        let mut memo: HashMap<u64, u64> = HashMap::new();
        memo.insert(0, 0);
        memo.insert(1, 1);
        Fibonacci { memo }
    }

    fn get(&mut self, n: u64) -> u64 {
        if let Some(&result) = self.memo.get(&n) {
            return result;
        }

        let result = self.get(n - 1) + self.get(n - 2);
        self.memo.insert(n, result);
        result
    }
}

fn main() {
    let mut fib = Fibonacci::new();
    println!("{}", (0..)
        .map(|i| fib.get(i))
        .take_while(|&n| n <= 4000000)
        .filter(|&n| n % 2 == 0)
        .sum::<u64>()
    );
}
