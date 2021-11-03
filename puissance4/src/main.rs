extern crate piston_window;
extern crate image as im;
extern crate vecmath;
use std::io::{Write,Read};
use std::net::TcpStream;
use piston_window::*;



fn main() {
    let opengl = OpenGL::V3_2;
    
    let size_tile = 100.0;
    const NB_COLUMN :i64 = 7;
    const NB_RAW :i64 = 6;
    let margin_x = size_tile + 20.0;
    let margin_y = size_tile + 20.0;
    let color_blue = [0.0, 0.0, 1.0, 1.0];
    let size_grille = math::margin_rectangle([margin_x, margin_y, size_tile*NB_COLUMN as f64, size_tile*NB_RAW as f64], 0.0);
    let mut game_finish : bool = false;
    let mut the_winner : Token = Token::NONE;

    let color_white = [1.0,1.0,1.0,1.0];
    let color_yellow = [1.0,0.8,0.0,1.0];
    let color_red = [1.0,0.0,0.0,1.0];

    let mut current_player = Token::YELLOW;

    let (width, height) = ((size_tile*(NB_COLUMN as f64)+ margin_x*2.0) as u32, (size_tile*(NB_RAW as f64)+ margin_y*2.0) as u32);

    let mut window: PistonWindow = WindowSettings::new("Puissance 4", (width, height))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_lazy(true);

    

    #[derive(Copy,Clone)]
    enum Token {
        NONE =0,
        YELLOW,
        RED
    }


    let mut grid = [[Token::NONE;NB_RAW as usize];NB_COLUMN as usize];


    println!("Tentative de connexion au serveur...");
	match TcpStream::connect("127.0.0.1:1234") {
		Ok(mut stream) => {
			println!("Connexion au serveur réussie !");
	        let stdout = std::io::stdout();
            let mut io = stdout.lock();
			write!(io, "write fait !\n");
            let buf = &mut [0; 10];
            match stream.read(buf) {
                Ok(received) => {
                    match buf[0] as char{
                        '2' => {
                            current_player = Token::RED;
                            println!("RED");}
                        '1' => {
                            current_player = Token::YELLOW;
                            println!("YELLOW");}
                        _ => {
                            println!("error color");
                        }
                    }

                    
                }
                Err(_) => {
                    println!("Client disconnected");
                    return;
                }
            }
		


    while let Some(e) = window.next() {
        if !game_finish{  
            game_finish = true;
            let input : char = 'a'; 
            let column_played = match input {
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
                grid = add_token(grid,column_played,current_player);
                the_winner = check_winner(grid);
            }                
            match the_winner {
                Token::RED | Token::YELLOW => {current_player=Token::NONE}
                Token::NONE => {}
            }
        }
        // if let Some(button) = e.press_args() {
        //     use piston_window::Button::Keyboard;

        //     if button == Keyboard(Key::R) {
        //         grid = [[Token::NONE;NB_RAW as usize];NB_COLUMN as usize];
        //         println!("Reset grid");
        //         if game_finish{
        //             current_player = Token::YELLOW;
        //             the_winner = Token::NONE;
        //             game_finish = false
        //         }
        //     }
        // }
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);            

            let outline_frame = math::margin_rectangle([0.0, 0.0, size_tile*NB_COLUMN as f64 + margin_x*2.0, size_tile*NB_RAW as f64+ margin_y*2.0], 0.0);
            let inline_frame = math::margin_rectangle([margin_x/2.0, margin_y/2.0, size_tile*NB_COLUMN as f64 + margin_x, size_tile*NB_RAW as f64 + margin_y], 0.0);
            
            match the_winner {
                Token::RED => {
                    rectangle(color_red, outline_frame, c.transform, g);
                    rectangle(color_white, inline_frame, c.transform, g);}
                Token::YELLOW => {
                    rectangle(color_yellow, outline_frame, c.transform, g);
                    rectangle(color_white, inline_frame, c.transform, g);}
                Token::NONE => {}
            }

            rectangle(color_blue, size_grille, c.transform, g);

            for (i,col) in grid.iter().enumerate() {       
                let c = c.trans(0.0, 0.0);
                for (j,cell) in col.iter().enumerate() {
                    let size_trou = [margin_x + size_tile /10.0 + size_tile * i as f64, margin_y + size_tile /10.0 + size_tile * j as f64, size_tile * 8.0/10.0, size_tile * 8.0/10.0];
                    let color_jeton;
                    match cell {
                        Token::NONE => {color_jeton = color_white},
                        Token::YELLOW => {color_jeton = color_yellow},
                        Token::RED => {color_jeton = color_red}
                    }
                    ellipse(color_jeton, size_trou, c.transform, g);
 
                }
            }


        });
        
    }} 
        Err(e) => {
            println!("La connexion au serveur a échoué : {}", e);
        }
    }

    fn add_token( mut grid:[[Token;NB_RAW as usize];NB_COLUMN as usize], column: i64, current_player: Token) -> 
        [[Token;NB_RAW as usize];NB_COLUMN as usize] {
            let mut count = NB_RAW-1 as i64;
            while count>=0{
                match grid[column as usize][count as usize] {
                    Token::NONE => {grid[column as usize][count as usize] = current_player;                    }
                    Token::YELLOW | Token::RED => {count-=1}
                }
            }
            return grid;
        }
        
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
}
