use rand::{Rng, thread_rng};
use sparse_merkle_tree::H256;
use crate::smt::SMT;

mod smt;

fn main() {
    let mut rng = thread_rng();
    let mut leaves = vec![];
    for _ in 0..5000 {
        let key: H256 = rng.gen::<[u8; 32]>().into();
        let value: H256 = rng.gen::<[u8; 32]>().into();
        leaves.push((key, value));
    }
    let mut smt = SMT::default();
    smt.update_all(leaves.clone()).expect("SMT update leaves error");

    let update_leaves = leaves[0..1000].to_vec();
    let merkle_proof = smt
        .merkle_proof(update_leaves.iter().map(|leave| leave.0).collect())
        .unwrap();
    let merkle_proof_compiled = merkle_proof
        .compile(update_leaves.clone())
        .unwrap();
    let proof: Vec<u8> = merkle_proof_compiled.into();
    // length: 112619
    println!("The proof of 1000 leaves: {:?}", proof.len());

    let update_leaves = leaves[0..100].to_vec();
    let merkle_proof = smt
        .merkle_proof(update_leaves.iter().map(|leave| leave.0).collect())
        .unwrap();
    let merkle_proof_compiled = merkle_proof
        .compile(update_leaves.clone())
        .unwrap();
    let proof: Vec<u8> = merkle_proof_compiled.into();
    // length: 24203
    println!("The proof of 100 leaves: {:?}", proof.len());


    let update_leaves = leaves[0..1].to_vec();
    let merkle_proof = smt
        .merkle_proof(update_leaves.iter().map(|leave| leave.0).collect())
        .unwrap();
    let merkle_proof_compiled = merkle_proof
        .compile(update_leaves.clone())
        .unwrap();
    let proof: Vec<u8> = merkle_proof_compiled.into();
    // length: 366
    println!("The proof of one leaf: {:?}", proof.len());
}
