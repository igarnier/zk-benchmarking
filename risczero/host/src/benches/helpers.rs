// TODO: is there a cleaner way to compute the proof size? Something directly
// exposed in the risc0 API?
// pub fn inner_receipt_size_bytes(proof: &risc0_zkvm::receipt::InnerReceipt) -> u32 {
//     let bytes: Vec<u8> = bincode::serialize(proof).unwrap();
//     bytes.len() as u32
// }

use risc0_zkvm::receipt::InnerReceipt::*;
use risc0_zkvm::receipt::SegmentReceipts;

pub fn inner_receipt_size_bytes(proof: &risc0_zkvm::receipt::InnerReceipt) -> u32 {
    match proof {
        Flat(SegmentReceipts(vec)) => {
            let segments = vec.len();
            println!("Number of segments = {}", segments)
        }
        Succinct(succinct) => {
            let size = succinct.seal.len();
            println!("Seal length: u32 elements = {}", size)
        }
        Fake => {}
    }
    let bytes: Vec<u8> = bincode::serialize(proof).unwrap();
    bytes.len() as u32
}
