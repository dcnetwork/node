use webrtc_util::conn::Conn;
use webrtc_util::Error;
use std::net::SocketAddr;
use std::future::Future;
use std::pin::Pin;
use std::boxed::Box;
use webrtc_util::conn::conn_udp_listener::UdpConn;
use webrtc::stun;
use tokio::net::UdpSocket;
use tokio::net;
use std::sync::Arc;
pub struct DcStunClient{
	// pub client: stun::client::Client
}

impl DcStunClient{

	pub fn new() -> Self{
		Self{}
	}
	pub async fn start(&self){

		let sock = UdpSocket::bind("0:0").await.expect("Failed to Bind Address.");
		let mut buf = [0; 1024];
		println!("Listening on: {:?}",sock.local_addr().unwrap());
		let server = "stun.l.google.com:19302";
		let x = net::lookup_host("stun.l.google.com:19302").await.unwrap();
		let v = x.last().unwrap();
		print!("Connecting to: [\x1b[34m{}\x1b[0m] [\x1b[34m{:?}\x1b[0m]", server,v);
		sock.connect(v).await.expect("Failed");
    	println!(" ... [\x1b[33mOk\x1b[0m]");
		let client_build = stun::client::ClientBuilder::new().with_conn(Arc::new(sock).clone());
		let client = client_build.build().expect("Error Bilding Client From ClientBuilder");

    	// loop {
     //   		let (len, addr) = sock.recv_from(&mut buf).await.unwrap();
     //    	println!("{:?} bytes received from {:?}", len, addr);
     //    	let len = sock.send_to(&buf[..len], addr).await.unwrap();
     //    	println!("{:?} bytes sent", len);
    	// }

	}

}
