use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;


fn handle_client(mut stream: TcpStream,adresse: &str) {
    let mut msg: Vec<u8> = Vec::new();
    write!(stream,"2");
    //println!("connecter + message envoyé");
    loop {
        let mut buf = &mut [0; 10];

        match stream.read(buf) {
            Ok(received) => {
                if received < 1 {
                    println!("Client disconnected {}",adresse);
                    return;
                }
                let mut x = 0;

                for c in buf {
                    if x>=received {
                        break;
                    }
                    x += 1;
                    if *c == '\n' as u8 {
                        println!("message reçu {} : {}", adresse, String::from_utf8(msg).unwrap());
                        stream.write(b"ok\n");
                        msg = Vec::new();
                    } else {
                        msg.push(*c);
                    }
                }
            }
            Err(_) => {
                println!("Client disconnected {}", adresse);
                return;
            }
        }
    }
}

fn main() {
    let listener =  TcpListener::bind("127.0.0.1:1234").unwrap();
    println!("En attente d'un client...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let adresse = match stream.peer_addr() {
                    Ok(addr) => format!("[adresse : {}]",addr),
                    Err(_) => "inconnue".to_owned()
                };
                println!("Nouveau client {}", adresse);

                thread::spawn(move|| {
                    handle_client(stream, &*adresse)
                });
            }
            Err(e) => {
                println!("La connexion du client a échoué : {}", e);
            }
        }
        println!("En attente d'un autre client...");
    }
}
