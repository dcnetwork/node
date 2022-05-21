
use node::stun::connect::*;
use node::config::init::*;
use signaling::call::*;
use tokio;
use hex;

#[tokio::main]
async fn main() {
    // initialize the node
    let node = DcNodeInit::new().await;
    // start the signaling listner
    start_signal(node.address.clone()).await;
    //
    let pbk = hex::encode(node.pub_key_vec);
    //
    let addr = hex::encode(node.address);
    //
    push_text_call(String::from("UNKNOWN"),addr,pbk).await;

}
