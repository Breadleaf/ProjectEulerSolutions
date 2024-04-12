// Problem Statement: https://projecteuler.net/problem=1

fn main() {
    println!(
        "{}",
        (0..1000)
            .filter(|i| i % 3 == 0 || i % 5 == 0)
            .sum::<i32>()
    );
}
