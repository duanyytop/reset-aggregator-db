use crate::smt::SMT;
use rand::{thread_rng, Rng};
use sparse_merkle_tree::H256;

mod smt;

fn main() {
    println!("Test the proof of 100 update leaves and 1000 history leaves");
    test_256_smt();
    test_48_smt();
}

fn test_256_smt() {
    let mut rng = thread_rng();
    let mut leaves = vec![];
    for _ in 0..1000 {
        let key: H256 = rng.gen::<[u8; 32]>().into();
        let value: H256 = rng.gen::<[u8; 32]>().into();
        leaves.push((key, value));
    }
    let mut smt = SMT::default();
    smt.update_all(leaves.clone())
        .expect("SMT update leaves error");

    let mut update_leaves = vec![];
    for _ in 0..100 {
        let key: H256 = rng.gen::<[u8; 32]>().into();
        let value: H256 = rng.gen::<[u8; 32]>().into();
        update_leaves.push((key, value));
    }
    let merkle_proof = smt
        .merkle_proof(update_leaves.iter().map(|leave| leave.0).collect())
        .unwrap();
    let merkle_proof_compiled = merkle_proof.compile(update_leaves.clone()).unwrap();
    let proof: Vec<u8> = merkle_proof_compiled.into();
    println!("The proof size of 256bit key: {:?}", proof.len());
}

fn test_48_smt() {
    let mut rng = thread_rng();
    let mut leaves = vec![];
    for _ in 0..1000 {
        let random = rng.gen::<[u8; 6]>();
        let mut key = [0u8; 32];
        key[26..].copy_from_slice(&random);

        let value: H256 = rng.gen::<[u8; 32]>().into();
        leaves.push((key.into(), value));
    }
    let mut smt = SMT::default();
    smt.update_all(leaves.clone())
        .expect("SMT update leaves error");

    let mut update_leaves = vec![];
    for _ in 0..100 {
        let random = rng.gen::<[u8; 6]>();
        let mut key = [0u8; 32];
        key[26..].copy_from_slice(&random);

        let value: H256 = rng.gen::<[u8; 32]>().into();
        update_leaves.push((key.into(), value));
    }
    let merkle_proof = smt
        .merkle_proof(update_leaves.iter().map(|leave| leave.0).collect())
        .unwrap();
    let merkle_proof_compiled = merkle_proof.compile(update_leaves.clone()).unwrap();
    let proof: Vec<u8> = merkle_proof_compiled.into();
    println!("The proof size of 48bit key: {:?}", proof.len());
}
