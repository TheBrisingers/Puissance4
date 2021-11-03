use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::mpsc;




fn handle_client(mut stream: TcpStream,adresse: &str, player_color: i64) {
    let mut msg: Vec<u8> = Vec::new();
    write!(stream,"{}",player_color);
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
    let mut player_color = 0;
    for stream in listener.incoming() {
        println!("start");
        match stream {
            Ok(stream) => {
                let (tx, rx) = mpsc::channel();
                let adresse = match stream.peer_addr() {
                    Ok(addr) => format!("[adresse : {}]",addr),
                    Err(_) => "inconnue".to_owned()
                };
                println!("Nouveau client {}", adresse);
                player_color +=1;
                thread::spawn(move|| {
                    handle_client(stream, &*adresse, player_color);
                    let val = String::from("hi");
                    tx.send(val).unwrap();
                });
                let received = rx.recv().unwrap();
                println!("Got: {}", received);
            }
            Err(e) => {
                println!("La connexion du client a échoué : {}", e);
            }
        }
        println!("En attente d'un autre client...");
       
    }

}
