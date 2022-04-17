use  stun::client::ClientBuilder;
use std::sync::Arc;
use tokio::net::{UdpSocket,self};

pub struct DcStunConnect {

}

impl DcStunConnect{
	pub async fn new() -> Self {
		Self{}
	}
}
