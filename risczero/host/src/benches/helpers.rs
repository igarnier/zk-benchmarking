// TODO: is there a cleaner way to compute the proof size? Something directly
// exposed in the risc0 API?
// pub fn inner_receipt_size_bytes(proof: &risc0_zkvm::receipt::InnerReceipt) -> u32 {
//     let bytes: Vec<u8> = bincode::serialize(proof).unwrap();
//     bytes.len() as u32
// }

pub fn inner_receipt_size_bytes(proof: &risc0_zkvm::receipt::InnerReceipt) -> u32 {
    match proof {
        risc0_zkvm::receipt::InnerReceipt::Flat(_) => {}
        risc0_zkvm::receipt::InnerReceipt::Succinct(succinct) => {
            let size = succinct.seal.len();
            println!("Seal length: u32 elements = {}", size)
        }
        risc0_zkvm::receipt::InnerReceipt::Fake => {}
    }
    let bytes: Vec<u8> = bincode::serialize(proof).unwrap();
    bytes.len() as u32
}
