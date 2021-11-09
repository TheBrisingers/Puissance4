/**
 * @file main.rs
 *
 * @brief This class has for purpose to be the server of the game
 *
 * @date 9th November 2021
 *
 * @authors Timothée Jouffrieau Jordy Laurent
 *
 * @version 1.0
 *
 * @copyright CCBY 4.0
 */

use std::net::{Shutdown,TcpListener, TcpStream};
use std::io::{Read};
use std::thread;
use std::sync::mpsc;
use std::process;

/*-----------------------------------------------Variables-------------------------------------------------------*/
/**
 * @var NB_COLUMN
 *
 * @brief Nombre de colonnes
 */
const NB_COLUMN :i64 = 7;

/**
 * @var NB_RAW
 *
 * @brief Nombre de rangées
 */
const NB_RAW :i64 = 6;

#[derive(Copy,Clone)]
    enum Token {
        NONE =0,
        YELLOW,
        RED
    }

/*-----------------------------------------------Functions-------------------------------------------------------*/

/**
 * Écoute en permanence le client
 */
fn handle_client(mut stream: TcpStream,adresse: &str, tx: mpsc::Sender::<u8>) {
    let mut msg: u8 = 'j' as u8;
    loop {
        let buf = &mut [0; 10];

        match stream.read(buf) {
            Ok(received) => {
                if received < 1 {
                    println!("Client disconnected {}",adresse);
                    println!("Partie terminée !");
                    process::exit(1);
                }
                let mut x = 0;
                for c in buf {
                    if x>=received {
                        break;
                    }
                    x += 1;
                    if *c == '\n' as u8 {
                        tx.send(msg);
                    } else {
                        msg = *c;
                    }
                }
            }
            Err(_) => {
                println!("Client disconnected {}", adresse);
                println!("Partie terminée !");
                process::exit(1);
            }
        }
    }
}

fn main() {

    let listener =  TcpListener::bind("127.0.0.1:1234").unwrap();
    let mut count = 0;
    let (tx1, rx1) = mpsc::channel::<u8>();
    let (tx2, rx2) = mpsc::channel::<u8>();
    let mut the_winner : Token = Token::NONE;

    let mut grid = [[Token::NONE;NB_RAW as usize];NB_COLUMN as usize];
    

    println!("En attente d'un client...");
    for stream in listener.incoming() {
        println!("{}",count);
        match stream {
            Ok(stream) => {
                if count < 2 {
                    let adresse = match stream.peer_addr() {
                        Ok(addr) => format!("[adresse : {}]",addr),
                        Err(_) => "inconnue".to_owned()
                    };
                    println!("Nouveau joueur {}", adresse);
                    
                    let tx;
                    if count ==0 {
                        tx = tx1.clone();
                    } else {tx = tx2.clone();}
                    count += 1;
                    thread::spawn(move|| {
                        handle_client(stream, &*adresse, tx);
                    });
                } else {
                    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
                }
                if count == 2 {
                    
                    loop {
                        show_grille(&grid);
                        
                        match rx1.recv(){
                            Ok(data) => {
                                let column_played = match data as char {
                                    'a' => {0},
                                    'z' => {1},
                                    'e' => {2},
                                    'r' => {3},
                                    't' => {4},
                                    'y' => {5},
                                    'u' => {6},
                                    _ => {-1}
                                };
                                if column_played != -1 {             
                                    grid = add_token(grid,column_played,Token::RED);
                                    the_winner = check_winner(grid);
                                }                
                                match the_winner {
                                    Token::RED => {show_grille(&grid); println!("Red win !!"); break}
                                    Token::YELLOW => {show_grille(&grid); println!("Yellow win !!"); break}
                                    Token::NONE => {}
                                }
                            },
                            Err(e) => println!("Une erreur s'est produite : {:?}", e)
                        }

                        show_grille(&grid);

                        match rx2.recv(){
                            Ok(data) => {
                                let column_played = match data as char {
                                    'a' => {0},
                                    'z' => {1},
                                    'e' => {2},
                                    'r' => {3},
                                    't' => {4},
                                    'y' => {5},
                                    'u' => {6},
                                    _ => {-1}
                                };
                                if column_played != -1 {             
                                    grid = add_token(grid,column_played,Token::YELLOW);
                                    the_winner = check_winner(grid);
                                }                
                                match the_winner {
                                    Token::RED => {show_grille(&grid); println!("Red win !!"); break}
                                    Token::YELLOW => {show_grille(&grid); println!("Yellow win !!"); break}
                                    Token::NONE => {}
                                }
                            },
                            Err(e) => println!("Une erreur s'est produite : {:?}", e)
                        }
                    }
                    break;
                }
            }
            Err(e) => {
                println!("La connexion du joueur a échoué : {}", e);
            }
        }
        println!("En attente d'un autre joueur...");    
    }    
}

/**
 * Montre la grille 
 */
fn show_grille(grid: &[[Token;NB_RAW as usize];NB_COLUMN as usize]) {
    let mut msg = String::from("A Z E R T Y U\n");
    for raw in 0..NB_RAW {
        for col in 0..NB_COLUMN {
            let cell = grid[col as usize][raw as usize];
            let color_jeton = match cell {
                Token::NONE => {"0 "},
                Token::YELLOW => {"Y "},
                Token::RED => {"R "}
            };
            msg = format!("{}{}", msg, color_jeton);

         }
         msg = format!("{}{}", msg, "\n");
     }
     println!("{}", msg);
    // }
}

/**
 * Ajoute un jeton dans la grille
 */
fn add_token( mut grid:[[Token;NB_RAW as usize];NB_COLUMN as usize], column: i64, current_player: Token) -> 
        [[Token;NB_RAW as usize];NB_COLUMN as usize] {
            let mut count = NB_RAW-1 as i64;
            while count>=0{
                match grid[column as usize][count as usize] {
                    Token::NONE => {grid[column as usize][count as usize] = current_player;
                    break;}
                    Token::YELLOW | Token::RED => {count-=1}
                }
            }
            return grid;
        }
        

/**
 * Vérifie si il y a un gagnant 
 */
fn check_winner(grid:[[Token;NB_RAW as usize];NB_COLUMN as usize]) ->Token{

        //verif columns
        let mut count_yellow;
        let mut count_red;
        for col in grid.iter() {
            count_red = 0 as i64;
            count_yellow = 0 as i64;
            for cell in col.iter() {
                match cell  {
                    Token::NONE => {count_yellow = 0; count_red = 0},
                    Token::YELLOW => {count_red = 0; count_yellow +=1},
                    Token::RED => {count_yellow = 0; count_red +=1}
                }
                if  count_red>=4 {
                    return Token::RED
                } else if count_yellow>=4{
                    return Token::YELLOW
                }
                
            }
        } 
        //verif raw
        for raw in 0..NB_RAW {
            count_red = 0 as i64;
            count_yellow = 0 as i64;
            for col in 0..NB_COLUMN {
                let cell = grid[col as usize][raw as usize];
                match cell  {
                    Token::NONE => {count_yellow = 0; count_red = 0},
                    Token::YELLOW => {count_red = 0; count_yellow +=1},
                    Token::RED => {count_yellow = 0; count_red +=1}
                }
                if  count_red>=4 {
                    return Token::RED
                } else if count_yellow>=4{
                    return Token::YELLOW
                }
            }
        }   

        //verif diagonal
        for diag in 0..=NB_COLUMN+NB_RAW-2 {
            count_red = 0 as i64;
            count_yellow = 0 as i64;
            for raw in 0..=diag {
                let col = diag - raw;
                if col<NB_COLUMN && raw<NB_RAW {
                    let cell = grid[col as usize][raw as usize];
                    match cell  {
                        Token::NONE => {count_yellow = 0; count_red = 0},
                        Token::YELLOW => {count_red = 0; count_yellow +=1},
                        Token::RED => {count_yellow = 0; count_red +=1}
                    }
                    if  count_red>=4 {
                        return Token::RED
                    } else if count_yellow>=4{
                        return Token::YELLOW
                    }
                }
            }
        }  
        //verif diagonal 2
        for diag in 0..=NB_COLUMN+NB_RAW-2 {
            count_red = 0 as i64;
            count_yellow = 0 as i64;
            for raw in 0..=diag {
                let col = diag - raw;
                if col<NB_COLUMN && raw<NB_RAW {
                    let cell = grid[col as usize][NB_RAW as usize - raw as usize -1];
                    match cell  {
                        Token::NONE => {count_yellow = 0; count_red = 0},
                        Token::YELLOW => {count_red = 0; count_yellow +=1},
                        Token::RED => {count_yellow = 0; count_red +=1}
                    }
                    if  count_red>=4 {
                        return Token::RED
                    } else if count_yellow>=4{
                        return Token::YELLOW
                    }
                }
            }
        }  
        return Token::NONE
    }