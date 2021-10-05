extern crate piston_window;
extern crate image as im;
extern crate vecmath;

use piston_window::*;



fn main() {
    let opengl = OpenGL::V3_2;
    let (width, height) = (1000, 800);
    let mut window: PistonWindow = WindowSettings::new("shapes", (width, height))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_lazy(true);

    let size_tile = 100.0;
    const NB_COLUMN :i64 = 7;
    const NB_RAW :i64 = 6;
    let margin_x = 20.0;
    let margin_y = 120.0;
    let blue = [0.0, 0.0, 1.0, 1.0];
    let size_grille = math::margin_rectangle([margin_x, margin_y, size_tile*NB_COLUMN as f64, size_tile*NB_RAW as f64], 0.0);
    let mut cursor_column = 0;
    let mut game_finish = false;

    let mut current_player = Jeton::YELLOW;

    #[derive(Copy,Clone)]
    enum Jeton {
        NONE =0,
        YELLOW,
        RED
    }


    let mut grid = [[Jeton::NONE;NB_RAW as usize];NB_COLUMN as usize];



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
                            Jeton::NONE => {grid[cursor_column as usize][count as usize] = current_player;
                                match current_player {
                                    //send to socket
                                    Jeton::RED => {current_player = Jeton::YELLOW}
                                    Jeton::YELLOW => {current_player = Jeton::RED}
                                    Jeton::NONE => {}
                                }
                                break;
                            }
                            Jeton::YELLOW | Jeton::RED => {count-=1}
                        }
                    }
                    //verif columns
                    let mut count_yellow;
                    let mut count_red;
                    for col in grid.iter() {
                        count_red = 0 as i64;
                        count_yellow = 0 as i64;
                        for cell in col.iter() {
                            match cell  {
                                Jeton::NONE => {count_yellow = 0; count_red = 0},
                                Jeton::YELLOW => {count_red = 0; count_yellow +=1},
                                Jeton::RED => {count_yellow = 0; count_red +=1}
                            }
                            if count_red == 4 {
                                println!("RED WIN column !!");
                                game_finish =true;
                                break;
                            }
                            else if count_yellow == 4 {
                                println!("YELLOW WIN column !!");
                                game_finish =true;
                                break;
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
                                Jeton::NONE => {count_yellow = 0; count_red = 0},
                                Jeton::YELLOW => {count_red = 0; count_yellow +=1},
                                Jeton::RED => {count_yellow = 0; count_red +=1}
                            }
                            if count_red == 4 {
                                println!("RED WIN raw !!");
                                game_finish =true;
                                break;
                            }
                            else if count_yellow == 4 {
                                println!("YELLOW WIN raw !!");
                                game_finish =true;
                                break;
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
                                    Jeton::NONE => {count_yellow = 0; count_red = 0},
                                    Jeton::YELLOW => {count_red = 0; count_yellow +=1},
                                    Jeton::RED => {count_yellow = 0; count_red +=1}
                                }
                                if count_red == 4 {
                                    println!("RED WIN diag 1!!");
                                    game_finish =true;
                                    break;
                                }
                                else if count_yellow == 4 {
                                    println!("YELLOW WIN diag 1!!");
                                    game_finish =true;
                                    break;
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
                                    Jeton::NONE => {count_yellow = 0; count_red = 0},
                                    Jeton::YELLOW => {count_red = 0; count_yellow +=1},
                                    Jeton::RED => {count_yellow = 0; count_red +=1}
                                }
                                if count_red == 4 {
                                    println!("RED WIN diag 2!!");
                                    game_finish =true;
                                    break;
                                }
                                else if count_yellow == 4 {
                                    println!("YELLOW WIN diag 2!!");
                                    game_finish =true;
                                    break;
                                }
                            }
                        }
                }  

                }
            };
            //efface la preview si la parti est fini
            if game_finish {current_player=Jeton::NONE}
        }
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
        
            let size_jeton= [margin_x + size_tile /10.0 + size_tile * cursor_column as f64, size_tile /10.0, size_tile * 8.0/10.0, size_tile * 8.0/10.0];
            let next_jeton_preview;
            match current_player {
                Jeton::NONE => {next_jeton_preview = [1.0, 1.0, 1.0, 1.0]},
                Jeton::YELLOW => {next_jeton_preview = [1.0, 0.8, 0.0, 1.0];},
                Jeton::RED => {next_jeton_preview = [1.0, 0.0, 0.0, 1.0];}
            }
            ellipse(next_jeton_preview, size_jeton, c.transform, g);
            
            rectangle(blue, size_grille, c.transform, g);

            for (i,col) in grid.iter().enumerate() {       
                let c = c.trans(0.0, 0.0);
                for (j,cell) in col.iter().enumerate() {
                    let size_trou = [margin_x + size_tile /10.0 + size_tile * i as f64, margin_y + size_tile /10.0 + size_tile * j as f64, size_tile * 8.0/10.0, size_tile * 8.0/10.0];
                    let color_jeton;
                    match cell {
                        Jeton::NONE => {color_jeton = [1.0, 1.0, 1.0, 1.0];},
                        Jeton::YELLOW => {color_jeton = [1.0, 0.8, 0.0, 1.0];},
                        Jeton::RED => {color_jeton = [1.0, 0.0, 0.0, 1.0];}
                    }
                    ellipse(color_jeton, size_trou, c.transform, g);
 
                }
            }

        });
        
    }
}
