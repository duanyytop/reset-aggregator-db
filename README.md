## smt-proof

`main.rs` 比较了不同数量叶子验证时所需要的 proof 的长度，每次执行数据都不完全一样，但是数量级是不变的。

## 方案设计

Witness 包含了合约需要验证的所有数据，其数据结构如下：

```
table RawTransaction {
    version:        Uint32,
    cell_deps:      CellDepVec,
    header_deps:    Byte32Vec,
    inputs:         CellInputVec,
    outputs:        CellOutputVec,
    outputs_data:   BytesVec,
}

table MerkleProof {
    indices: Uint32Vec,
    lemmas: H256Vec,
}

table TransactionProof {
    witnesses_root: H256,
    proof: MerkleProof,
}


table ClaimCotaNFTV2Entries {
  hold_keys: HoldCotaNFTKeyVec,
  hold_values: HoldCotaNFTValueVec,
  claim_keys: ClaimCotaNFTKeyVec,
  claim_values: ClaimCotaNFTValueVec,
  proof: Bytes,
  withdrawal_proof: Bytes,
  action: Bytes,
  raw_tx: RawTransaction,
  output_index: Uint32,
  tx_proof: TransactionProof,
}
```

- 重点关注 `ClaimCotaNFTV2Entries`，其中包含了 smt 需要验证的原始叶子信息，即 `hold_keys/hold_values/claim_keys/claim_values`,通过这些数据可以计算得到 smt 需要验证的 leaves。
- proof 是 claim 交易本身的 proof，即 hold/claim 叶子被验证时所需的 proof
- withdraw_proof 代表之前 sender withdraw 给当前用户的 smt proof，这个 proof 需要根据 sender 的 merkle tree 计算得到
- action 代表了这个交易的行为，提供了可视化的数据，可以先不关注
- raw_tx/output_index/tx_proof 三者主要是为了验证 withdraw 所在的 tx 在历史的某个区块中，具体验证方式，下面会详细说明。

### header_deps 验证 tx_proof 并得到 withdraw smt root

- header_deps 中存放的是 withdraw tx 所在的 block hash，合约可以由此获得该 block 的 header，其中 header 包含了 transactions_root（The hash of concatenated transaction hashes CBMT root and transaction witness hashes CBMT root.）
- 通过 ckb node 的 rpc 方法 get_transaction_proof 可以获得某个 tx 在 block 中的 proof 数据（TransactionProof），其中包含了 tx 本身的 proof 以及 block 中所有 tx.witnesses 的 CBMT root
- 通过 raw_tx（tx_hash） / tx_proof / transactions_root，再加上所需的 root，三者即可验证某个交易是否在某个 block 中，验证方式参考 CKB rpc verify_transaction_proof 的做法
- 通过 raw_tx / output_index 即可获得所需的 cell（此场景为 cota cell），从 cell data 中可以获得 smt root，此 root 即为 withdraw proof 所需要的 smt root

### withdraw_proof

基于今天下午的讨论结果，这里的 withdraw_proof 是之前 withdraw_tx 中的 proof，即包含多个所需验证 NFT 叶子的 proof，通过上面的当时可以获得 withdraw smt root，所以还需要提供 smt 验证所需要的原始 leaves，为了降低 leaves 的数据大小，只会提供 claim 交易所需要的 withdraw leaf，其他的无关 leaf 只会提供 H256 格式 (key, value)，leaves 数据的拼装是在链外完成的，链上合约只需要根据这些 leaf 恢复出完整的 leaves 即可。