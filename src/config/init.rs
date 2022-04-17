#![allow(unused_imports)]

use std::fs::*;
use std::io::prelude::*;
use std::io::{self,Error};
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::path::{Path,PathBuf};
use std::thread::spawn;
use std::sync::Arc;
use std::thread;
// ##########################################
use openssl::bn::BigNumContext;
use openssl::pkey::PKey;
use openssl::nid::Nid;
use openssl::ec::*;
use openssl::symm::{encrypt, Cipher};
use openssl::rand::rand_bytes;
use openssl::ecdsa::EcdsaSig;
use openssl::pkey::{Public,Private};
use openssl::bn::BigNum;
use openssl::string::OpensslString;
use hex;
use std::env::var;
// ##########################################
use sha3::{Digest, Keccak256,Sha3_256};
// ##########################################
use serde::*;
//
use reqwest;
//
use tokio::net::UdpSocket;
use tokio::net;
//###########################################
#[derive(Debug,Deserialize,Default,Clone)]
pub struct InfoData {
    pub ip:String ,
    city:String ,
    region:String ,
    country:String ,
    loc:String ,
    org:String ,
    postal:String ,
    timezone:String
}


//###########################################

struct ClientConfig{
	group:         EcGroup,
	key:           EcKey<Private>,
	pub_key_repr:  OpensslString,
	pub_key_vec:   Vec<u8>
}
//###########################################
impl ClientConfig{

	pub fn new() -> Self {
		// The Curve
		let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
		// Generate a Private Key from the curve
		let key = EcKey::generate(&group).unwrap();
		//
		let mut ctx = openssl::bn::BigNumContext::new().unwrap();
		//
		let bytes = key.public_key().to_bytes(&group,PointConversionForm::UNCOMPRESSED, &mut ctx).unwrap();
		//
		let pub_key_repr = BigNum::from_slice(&bytes).unwrap().to_hex_str().unwrap();
		//
		let pub_key_vec = key.public_key_to_pem().unwrap();

		// return the struct
		ClientConfig{
			group,
			key,
			pub_key_repr,
			pub_key_vec
		}

	}
	// get the curve group name
	pub fn get_group(&self) -> &EcGroup {

		&self.group
		/*EOF*/

	}
	// create a public.pem file
	pub fn dump_pub_pem(&self,filename:&Path){
		let mut f = File::create(&filename).unwrap();
		println!("\x1b[1m\x1b[28m[\x1b[31mCONFIG\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> WRITING \x1b[1m\x1b[33mPUBLIC\x1b[0m KEY TO PEM FILE");
		f.write(&self.pub_key_vec);
		/*EOF*/

	}
	// create a private.pem file
	pub fn dump_prv_pem(&self,filename:&Path){
		let mut f = File::create(&filename).unwrap();
		let mut buffer = String::new();
    	let mut stdin = io::stdin();
    	print!("\x1b[1m\x1b[28m[\x1b[31mCONFIG\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> ENTER PASSWORD FOR \x1b[1m\x1b[33mPRIVATE\x1b[0m KEY: ");
    	io::stdout().flush();
    	stdin.read_line(&mut buffer);
    	buffer.pop();
		let c = self.key.private_key_to_pem_passphrase(Cipher::aes_128_cbc(),buffer.as_bytes()).unwrap();
		println!("\x1b[1m\x1b[28m[\x1b[31mCONFIG\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> WRITING \x1b[1m\x1b[33mPRIVATE\x1b[0m KEY TO PEM FILE");
		f.write(&c);
		/*EOF*/

	}
}

//###########################################
pub struct DcNodeInit{

    pub address: Vec<u8>,          // server's address
	pub pubkey: EcKey<Public>,     // pubkey of server
	pub pub_key_vec:Vec<u8>,
    pub ip: InfoData
	/*EOS*/
}
//
impl DcNodeInit{

    pub async fn new() -> Self {
        let mut pubpath = PathBuf::from(var("HOME").unwrap());
		pubpath.push(".decchat/public.pem");

		let mut prvpath = PathBuf::from(var("HOME").unwrap());
		prvpath.push(".decchat/private.pem");

		let mut dir = PathBuf::from(var("HOME").unwrap());
		dir.push(".decchat");

		if !prvpath.as_path().exists() || !pubpath.as_path().exists(){
	    	if !dir.as_path().exists() {
				DirBuilder::new().recursive(true).create(dir.as_path().to_str().unwrap());
	    	}
	    	println!("\x1b[1m\x1b[28m[\x1b[31mCONFIG\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> \x1b[1m\x1b[32mGENERATING KEYS\x1b[0m");
			let tmp = ClientConfig::new();
			tmp.dump_pub_pem(pubpath.as_path());
			tmp.dump_prv_pem(prvpath.as_path());
		}
    	//
		let mut pub_file = File::open(pubpath.as_path()).unwrap();
		//
		let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
		//
		let mut ctx = openssl::bn::BigNumContext::new().unwrap();
		//
		let mut buf = String::new();
		// get public key
		pub_file.read_to_string(&mut buf);buf.pop();
		// get private key
		let buf_vec = buf.as_bytes();
		//
		let pubkey = EcKey::public_key_from_pem(&buf_vec).unwrap();
		//
		let bytes = pubkey.public_key().to_bytes(&group, PointConversionForm::UNCOMPRESSED, &mut ctx).unwrap();
		// keccak hasher
		let mut hasher = Keccak256::new();
		//
		hasher.update(&bytes);
		//
		let v = hasher.finalize();

        println!("\x1b[1m\x1b[28m[\x1b[33mINFO\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> YOUR \x1b[1m\x1b[32mADDRESS\x1b[0m <\x1b[1m\x1b[33m 0x{} \x1b[0m>",BigNum::from_slice(&v).unwrap().to_hex_str().unwrap());

        let body = reqwest::get("https://ipinfo.io/json?").await.unwrap().json::<InfoData>().await.unwrap();

        println!("\x1b[1m\x1b[28m[\x1b[33mINFO\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> YOUR \x1b[1m\x1b[32mNODE IP\x1b[0m <\x1b[1m\x1b[33m {} \x1b[0m>",body.ip);

		Self{
			address: v.to_vec(),
			pubkey,
			pub_key_vec: Vec::from(buf_vec),
            ip:body
        }
 		/*EOF*/
       
    }
    // sign a message
	pub fn sign( &self, msg: &[u8], password: &[u8] ) -> Vec<u8> {
		//
		let mut prvpath = PathBuf::from(var("HOME").unwrap());
		prvpath.push(".decchat/private.pem");

		let mut pbuf = String::new();
		//
		let mut prv_file = File::open(prvpath.as_path()).unwrap();
		//
		prv_file.read_to_string(&mut pbuf);pbuf.pop();
		//
		let pbuf_vec = pbuf.as_bytes();
		//
		let prvkey = EcKey::private_key_from_pem_passphrase(&pbuf_vec,password).unwrap();
		//
		let signature = EcdsaSig::sign(msg,&*prvkey).unwrap();
		//
		println!("Signature generated: \x1b[31m{}\x1b[0m",BigNum::from_slice(&signature.to_der().unwrap()).unwrap().to_hex_str().unwrap());
		// @return
		signature.to_der().unwrap()
		/*EOF*/

	}
	// verify a message
	pub fn verify( &self ,hash: &[u8], sig: &[u8] ,cpub: &EcKey<Public> ) -> bool {
		
		let signature = EcdsaSig::from_der(sig).unwrap();
		signature.verify(hash,&*cpub).unwrap()
		/*EOF*/
	}

	//
	pub fn shared_key(&self,opkey:&EcKey<Public>) -> EcKey<Public> {
		let mut prvpath = PathBuf::from(var("HOME").unwrap());
		prvpath.push(".decchat/private.pem");

		let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
		//
		let mut point = EcPoint::new(&group).unwrap();
		//
		let mut ctx = BigNumContext::new().unwrap();
		//
		let bytes = opkey.public_key().to_bytes(&group,PointConversionForm::UNCOMPRESSED, &mut ctx).unwrap();
		//
		let point1 = EcPoint::from_bytes(&group, &bytes, &mut ctx).unwrap();
		//
		let mut pbuf = String::new();
		//
		let mut prv_file = File::open(prvpath.as_path()).unwrap();
		//
		prv_file.read_to_string(&mut pbuf);pbuf.pop();
		//
		let pbuf_vec = pbuf.as_bytes();
		//
		let prvkey = EcKey::private_key_from_pem_passphrase( &pbuf_vec, "123".as_bytes() ).unwrap();
		//
		point.mul(&group,&*point1,&prvkey.private_key(), &mut ctx);

		let sh = EcKey::from_public_key(&group,&*point).unwrap();

		let bytes = sh.public_key().to_bytes(&group,PointConversionForm::UNCOMPRESSED, &mut ctx).unwrap();

		println!("\x1b[1m\x1b[28m[\x1b[33mINFO\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> sharedkey generated <\x1b[33m{}\x1b[0m>",BigNum::from_slice(&bytes[33..]).unwrap().to_hex_str().unwrap());

		sh
		/*EOF*/
	}
	//
    pub async fn start(&mut self) {

        let sock = UdpSocket::bind("0:0").await.expect("Failed to Bind Address.");
		let mut buf = [0; 1024];
	
        println!("\x1b[1m\x1b[28m[\x1b[34mSERVER\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> Listening on \x1b[1m\x1b[35m{:?}\x1b[0m",sock.local_addr().unwrap());		
		let server = "stun.l.google.com:19302";
		let x = net::lookup_host("stun.l.google.com:19302").await.unwrap();
		let v = x.last().unwrap();
		
		print!("\x1b[1m\x1b[28m[\x1b[34mSERVER\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> Connecting to  [\x1b[1m\x1b[34m{}\x1b[0m] [\x1b[1m\x1b[34m{:?}\x1b[0m]", server,v);

		// sock.connect(v).await.expect("Failed");
    	print!(" [\x1b[1m\x1b[33mOk\x1b[0m]\n");
		
    	loop {
       		let (len, addr) = sock.recv_from(&mut buf).await.unwrap();
        	println!("{:?} bytes received from {:?}", len, addr);
        	let len = sock.send_to(&buf[..len], addr).await.unwrap();
        	println!("{:?} bytes sent", len);
    	}
	/*EOF*/
    }

}

