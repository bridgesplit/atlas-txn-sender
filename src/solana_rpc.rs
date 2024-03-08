use solana_sdk::{clock::UnixTimestamp, commitment_config::CommitmentConfig};
use tonic::async_trait;

#[async_trait]
pub trait SolanaRpc: Send + Sync {
    fn get_next_slot(&self) -> Option<u64>;
    // return block_time if confirmed, None otherwise
    async fn confirm_transaction(&self, signature: String) -> Option<UnixTimestamp>;
    async fn confirm_transaction_with_commitment(&self, signature: String, commitment_config: CommitmentConfig) -> Option<UnixTimestamp>;

}
