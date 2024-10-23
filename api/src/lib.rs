use core::str;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

const INIT:[u8; 11]= [0xAA, 0xAA, 0xFF, 0x08, 0xC1, 0x02, 0x05, 0x00, 0x00, 0xCF, 0x73];

struct Payload{
    card_hex:String
}
impl Payload {
    fn parsing(buffer:Vec<u8>)->Self{
        let mut card_hex = String::new();
        if buffer.len()==26{
            buffer.iter().map(|i|{
                card_hex.push_str(format!("{:#X}",i).as_str());
            });
        }
        Self{
            card_hex
        }
    }
}

pub fn test_thread(

){
    thread::spawn(move||{
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut stream = TcpStream::connect("192.168.0.200:200").unwrap();
            stream.write(&INIT).unwrap();
            thread::sleep(Duration::from_secs(1));
            stream.write(&INIT).unwrap();
            let mut buffer = vec![0; 128];
            loop{
                if let Ok(data)=stream.read(&mut buffer){
                    
                }
            }
        })
    });
}