use core::str;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;
use tracing::{info,warn};
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
    str_mem:Arc<Mutex<Vec<String>>>
){
    thread::spawn(move||{
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut stream = TcpStream::connect("192.168.0.200:200").unwrap();
            info!("OPEN SOCKET IP : 192.168.0.200:200");
            stream.write(&INIT).unwrap();
            thread::sleep(Duration::from_secs(1));
            stream.write(&INIT).unwrap();
            let mut hex_init = String::new();
            INIT.map(|x|
                {
                    hex_init.push_str(format!("{:#X},",x).as_str());
                }
            );
            info!("SEND SIGNAL : {:?}",hex_init);
            let mut buffer = vec![0; 128];
            loop{
                let mut buffer = vec![0; 128];
                if let Ok(data) = stream.read(&mut buffer){
                    if buffer[0]==0xAA&&buffer[1]==0xAA&&buffer[2]==0xFF&&buffer[3]==0x18{
                        // let asd =format!("{:#X}",buffer[2]);
                        let mut hex_code = String::new();
                        for (i,data) in buffer.iter().enumerate(){
                            match i {
                                0..25=>hex_code.push_str(format!("{:#X},",*data).as_str()),
                                26=>hex_code.push_str(format!("{:#X}",*data).as_str()),
                                _=>{
                                }
                            }
                        }
                        // buffer.iter().enumerate().map(|(i,data)|{
                        // });
                        (*str_mem.lock().unwrap()).push(hex_code);
                        println!("{:?}",buffer);
                    }
                }
                // (*str_mem.lock().unwrap()).push(format!("{:#X}",buffer[2]));
                // println!("{:?}",buffer);
                // thread::sleep(Duration::from_millis(1));
            }
        })
    });
}