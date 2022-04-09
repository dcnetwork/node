use webrtc_util::conn::Conn;
use webrtc_util::Error;
use std::net::SocketAddr;
use std::future::Future;
use std::pin::Pin;
use std::boxed::Box;
use webrtc_util::conn::conn_udp_listener::UdpConn;
use webrtc::stun;

pub struct DcStunConnect{
	pub client: stun::Client
}

impl DcStunConnect{
	pub fn new()-> Self{
		stun::ClientBuilder::with_conn();

		Self{}
	}
}
