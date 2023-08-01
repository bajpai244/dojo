use jsonrpsee::core::{async_trait, Error};
use katana_core::accounts::Account;
use katana_core::sequencer::Sequencer;

use crate::api::katana::{KatanaApiError, KatanaApiServer};

pub struct KatanaApi<S> {
    sequencer: S,
}

impl<S> KatanaApi<S>
where
    S: Sequencer + Send + 'static,
{
    pub fn new(sequencer: S) -> Self {
        Self { sequencer }
    }
}

#[async_trait]
impl<S> KatanaApiServer for KatanaApi<S>
where
    S: Sequencer + Send + Sync + 'static,
{
    async fn generate_block(&self) -> Result<(), Error> {
        self.sequencer.backend().generate_latest_block().await;
        self.sequencer.backend().generate_pending_block().await;
        Ok(())
    }

    async fn next_block_timestamp(&self) -> Result<u64, Error> {
        Ok(self.sequencer.backend().block_context.read().block_timestamp.0)
    }

    async fn set_next_block_timestamp(&self, timestamp: u64) -> Result<(), Error> {
        self.sequencer
            .backend()
            .set_next_block_timestamp(timestamp)
            .await
            .map_err(|_| Error::from(KatanaApiError::FailedToChangeNextBlockTimestamp))
    }

    async fn increase_next_block_timestamp(&self, timestamp: u64) -> Result<(), Error> {
        self.sequencer
            .backend()
            .increase_next_block_timestamp(timestamp)
            .await
            .map_err(|_| Error::from(KatanaApiError::FailedToChangeNextBlockTimestamp))
    }

    async fn predeployed_accounts(&self) -> Result<Vec<Account>, Error> {
        Ok(self.sequencer.backend().predeployed_accounts.accounts.clone())
    }
}