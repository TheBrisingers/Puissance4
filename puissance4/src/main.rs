extern crate piston_window;
extern crate image as im;
extern crate vecmath;
use std::io::{Write,Read,stdin,stdout};
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
    let mut cursor_column = 0;
    let mut game_finish = false;
    let mut the_winner : Token = Token::NONE;

    let mut my_turn = false;

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
            if let Some(pos) = e.mouse_cursor_args() {
                cursor_column = ((pos[0] as f64-margin_x)/size_tile) as i64;
                if cursor_column>=NB_COLUMN {
                    cursor_column =NB_COLUMN-1;
                }
            }
            if let Some(button) = e.press_args() {
                if button == Button::Mouse(MouseButton::Left) {
                    let mut count = NB_RAW-1 as i64;
                    while count>=0{
                        match grid[cursor_column as usize][count as usize] {
                            Token::NONE => {grid[cursor_column as usize][count as usize] = current_player;
                                match current_player {
                                    //send to socket
                                    Token::RED => {
                                        if check_winner(grid){
                                            game_finish = true;
                                            the_winner = Token::RED;
                                        }
                                        //current_player = Token::YELLOW; 
                                        write!(stream, "jeton rouge\n");
                                    },
                                    Token::YELLOW => {
                                        if check_winner(grid){
                                            game_finish = true;
                                            the_winner = Token::YELLOW;
                                        }
                                        //current_player = Token::RED;
                                        write!(stream, "jeton jaune\n");
                                    },
                                    Token::NONE => {}
                                    
                                }
                                break;
                            }
                            Token::YELLOW | Token::RED => {count-=1}
                        }
                    }                    

                }
            };
            //efface la preview si la parti est fini
            if game_finish {
                current_player=Token::NONE}
        }
        if let Some(button) = e.press_args() {
            use piston_window::Button::Keyboard;

            if button == Keyboard(Key::R) {
                grid = [[Token::NONE;NB_RAW as usize];NB_COLUMN as usize];
                println!("Reset grid");
                if game_finish{
                    current_player = Token::YELLOW;
                    the_winner = Token::NONE;
                    game_finish = false
                }
            }
        }
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);

                
            let size_jeton= [margin_x + size_tile /10.0 + size_tile * cursor_column as f64, size_tile /10.0, size_tile * 8.0/10.0, size_tile * 8.0/10.0];
            let next_jeton_preview;
            match current_player {
                Token::NONE => {next_jeton_preview = color_white},
                Token::YELLOW => {next_jeton_preview = color_yellow},
                Token::RED => {next_jeton_preview = color_red}
            }
            ellipse(next_jeton_preview, size_jeton, c.transform, g);
            

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

    fn check_winner(grid:[[Token;NB_RAW as usize];NB_COLUMN as usize]) ->bool{

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
                if  count_red>=4 || count_yellow>=4 {
                    return true
                };
                
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
                if  count_red>=4 || count_yellow>=4 {
                    return true
                };
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
                    if  count_red>=4 || count_yellow>=4 {
                        return true
                    };
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
                    if count_red>=4 || count_yellow>=4 {
                        return true
                    }
                }
            }
        }  
        return false
    }
}
