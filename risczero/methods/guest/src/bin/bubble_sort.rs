#![no_main]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn bubble_sort<T: PartialOrd>(array: &mut Vec<T>) -> () {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j + 1] < array[j] {
                array.swap(j, j + 1);
            }
        }
    }
}

pub fn main() {
    let mut a: Vec<u64> = env::read();
    bubble_sort(&mut a);
    env::commit(&a);
}
