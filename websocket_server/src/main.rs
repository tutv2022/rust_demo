use std::net::TcpListener;
use tungstenite::{accept, Message};
use std::thread::spawn;
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
use rand::rngs::OsRng;

/// A WebSocket echo server
fn main () {

    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    
    for stream in server.incoming() {
        spawn (move || {

            let mut websocket = accept(stream.unwrap()).unwrap();

            let privateKeyFileName = "./serverPrivate.key";

            let publicKeyFileName = "./serverPublic.key";
            
            let private_key : RsaPrivateKey = pkcs1::DecodeRsaPrivateKey::read_pkcs1_pem_file(&privateKeyFileName).unwrap();
        
            let publicKeyData = std::fs::read_to_string(publicKeyFileName).unwrap();

            let mut clientPublicKey : Option<RsaPublicKey> = None;

            let mut secureTransfer = false;

            let mut rng = OsRng{};

            loop {

                let msg = websocket.read_message().unwrap();

                if msg.is_binary() || msg.is_text() {

                    let strMsg = unsafe {
                        String::from_utf8_unchecked(msg.clone().into_data())
                    };

                    if secureTransfer == false {

                        if strMsg.find("Exchange") == Some(0) {

                            let exchangeResponseMsg = format!("ExchangeServerPublicKey: {}", publicKeyData.clone());

                            let mut iter = strMsg.split(":");

                            iter.next();

                            let clientPublicKeyData = iter.next().unwrap().trim();

                            println!("Server received: {}",clientPublicKeyData);

                            if clientPublicKey == None {

                                clientPublicKey = Some(pkcs1::DecodeRsaPublicKey::from_pkcs1_pem(&clientPublicKeyData).unwrap());
                            }

                            websocket.write_message(Message::Text(exchangeResponseMsg)).unwrap();

                            secureTransfer = true;

                        }

                    } else {

                        let strData = msg.clone().into_data();

                        let dec_data = private_key.decrypt(Pkcs1v15Encrypt, &strData).expect("failed to decrypt");

                        let strReceivedData = std::str::from_utf8(&dec_data).unwrap();

                        println!("{}",strReceivedData);

                        if strReceivedData.find("Authenticating") == Some(0) {

                            let mut iter = strReceivedData.split(" ");

                            iter.next().unwrap();
                            
                            let userName = iter.next().unwrap().trim();

                            let passWord = iter.next().unwrap().trim();

                            if userName == "tutv1988@gmail.com" && passWord == "123456" {

                                let data = b"Authenticated: OK";

                                let pubKey = clientPublicKey.clone().unwrap();
                    
                                let enc_data = pubKey.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    
                                websocket.write_message(Message::Binary(enc_data)).unwrap();
    

                            }
                            
                        } else if dec_data == b"Hello Devr" {

                            let data = b"ACK";

                            let pubKey = clientPublicKey.clone().unwrap();
                
                            let enc_data = pubKey.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");

                            println!("{}",strReceivedData);

                            websocket.write_message(Message::Binary(enc_data)).unwrap();

                        } else { // echo

                            websocket.write_message( msg ).unwrap();

                        }

                    }
                   
                }
            }
        });
    }
}