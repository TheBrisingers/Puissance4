use std::net::TcpStream;
use std::io::{Write,Read,stdin,stdout};

fn get_entry() -> String {
	let mut buf = String::new();

	stdin().read_line(&mut buf);
	buf.replace("\n","").replace("\r","")
}

fn exchange_with_server(mut stream: TcpStream) {
	let stdout = std::io::stdout();
	let mut io = stdout.lock();
	let mut buf = &mut [0; 3];
	println!("Choisissez votre colonne avec AZERTYU");
	println!("Entrer 'quit' quand vous voulez quitter une partie");
	loop {
		write!(io, ">");
		io.flush();
		match &*get_entry() {
			"quit" => {
				println!("bye !");
				return;
			}
			line => {
				write!(stream, "{}\n", line);
				
			}
		}
	}

}

fn main() {
	println!("Tentative de connexion au serveur...");
	match TcpStream::connect("127.0.0.1:1234") {
		Ok(stream) => {
			println!("Connexion au serveur réussie !");
			exchange_with_server(stream);
		} 
		Err(e) => {
			println!("La connexion au serveur a échoué : {}", e);
		}
	}
}