use risc0_zkvm::guest::env;
use sha2::{Sha256, Digest};

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let zkvm_input: core::HashingData = env::read();

    let mut hasher = Sha256::new();
    hasher.update(zkvm_input.input);
    let output: [u8; 32] = hasher.finalize().into();

    // write public output to the journal
    env::commit(&(zkvm_input.output == output));
}
