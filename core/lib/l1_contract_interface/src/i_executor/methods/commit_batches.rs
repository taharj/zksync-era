// TODO: Remove once integrated into batch commit
#![allow(dead_code)]

use zksync_types::{commitment::L1BatchWithMetadata, ethabi::Token};

use crate::{
    i_executor::structures::{CommitBatchInfo, StoredBatchInfo},
    Tokenizable, Tokenize,
};

/// Input required to encode `commitBatches` call.
#[derive(Debug, Clone)]
pub struct CommitBatches {
    pub last_committed_l1_batch: L1BatchWithMetadata,
    pub l1_batches: Vec<L1BatchWithMetadata>,
}

impl Tokenize for CommitBatches {
    fn into_tokens(self) -> Vec<Token> {
        let stored_batch_info = StoredBatchInfo(&self.last_committed_l1_batch).into_token();
        let l1_batches_to_commit = self
            .l1_batches
            .iter()
            .map(|batch| CommitBatchInfo(batch).into_token())
            .collect();

        vec![stored_batch_info, Token::Array(l1_batches_to_commit)]
    }
}

impl CommitBatches {
    fn into_tokens_blobs(self, number_of_blobs: usize) -> Vec<Token> {
        let stored_batch_info = StoredBatchInfo(&self.last_committed_l1_batch).into_token();
        let l1_batches_to_commit = self
            .l1_batches
            .iter()
            .map(|batch| CommitBatchInfo(batch).into_tokens_blobs(number_of_blobs))
            .collect();

        vec![stored_batch_info, Token::Array(l1_batches_to_commit)]
    }

    fn into_tokens_calldata(self) -> Vec<Token> {
        let stored_batch_info = StoredBatchInfo(&self.last_committed_l1_batch).into_token();
        let l1_batches_to_commit = self
            .l1_batches
            .iter()
            .map(|batch| CommitBatchInfo(batch).into_tokens_calldata())
            .collect();

        vec![stored_batch_info, Token::Array(l1_batches_to_commit)]
    }
}
