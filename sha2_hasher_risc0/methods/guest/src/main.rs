use risc0_zkvm::guest::env;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: Vec<u8> = env::read();
    let output: Vec<u8> = env::read();

    let hash = sha::Impl::hash_bytes(&data);
    env::commit(hash)
    // TODO: do something with the input

    // write public output to the journal
    env::commit(&input);
}
