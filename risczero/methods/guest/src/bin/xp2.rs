#![no_main]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn pow(n: u64, m: u64) -> u64 {
    let mut acc = 1;
    for _i in 1..=n {
        acc *= 3;
    }
    acc &= m;
    acc
}
pub fn main() {
    let a: u64 = env::read();
    let result = pow(a, 9723893);
    env::commit(&result);
}
