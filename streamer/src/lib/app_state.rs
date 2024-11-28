use solana_client::{nonblocking::pubsub_client::PubsubClient, rpc_client::RpcClient};

pub struct AppState {
    pub rpc_client: RpcClient,
    pub ps_client: PubsubClient
}