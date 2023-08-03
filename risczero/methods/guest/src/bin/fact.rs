#![no_main]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn fibo(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}
pub fn main() {
    let a: u64 = env::read();
    let result = fibo(a);
    env::commit(&result);
}
