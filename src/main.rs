use node::stun::connect::*;
use tokio;
#[tokio::main]
async fn main() {
    DcStunClient::new().start().await;
}
