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

    let taille_case = 100.0;
    const nb_colonne :i64 = 7;
    const nb_ligne :i64 = 6;
    let marge_x = 20.0;
    let marge_y = 120.0;
    let blue = [0.0, 0.0, 1.0, 1.0];
    let taille_grille = math::margin_rectangle([marge_x, marge_y, taille_case*nb_colonne as f64, taille_case*nb_ligne as f64], 0.0);
    let mut cursor_colonne = 0;

    #[derive(Copy,Clone)]
    enum Jeton {
        NONE =0,
        YELLOW,
        RED
    }


    let mut grid = [[Jeton::NONE;nb_ligne as usize];nb_colonne as usize];



    while let Some(e) = window.next() {
        if let Some(pos) = e.mouse_cursor_args() {
            cursor_colonne = ((pos[0] as f64-marge_x)/taille_case) as i64;
            if cursor_colonne>=nb_colonne {
                cursor_colonne =nb_colonne-1;
            }
        }
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                let mut count = nb_ligne-1 as i64;
                while count>=0{
                    match grid[cursor_colonne as usize][count as usize] {
                        Jeton::NONE => {grid[cursor_colonne as usize][count as usize] = Jeton::YELLOW;
                            break;}
                        Jeton::YELLOW | Jeton::RED => {count-=1}
                    }
                }                
            }
        };
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
        
            let taille_jeton= [marge_x + taille_case /10.0 + taille_case * cursor_colonne as f64, taille_case /10.0, taille_case * 8.0/10.0, taille_case * 8.0/10.0];
            //let red = [1.0, 0.0, 0.0, 1.0];
            let yellow = [1.0, 0.8, 0.0, 1.0];
            ellipse(yellow, taille_jeton, c.transform, g);
            
            rectangle(blue, taille_grille, c.transform, g);

            for (i,col) in grid.iter().enumerate() {       
                let c = c.trans(0.0, 0.0);
                for (j,cell) in col.iter().enumerate() {
                    let taille_trou = [marge_x + taille_case /10.0 + taille_case * i as f64, marge_y + taille_case /10.0 + taille_case * j as f64, taille_case * 8.0/10.0, taille_case * 8.0/10.0];
                    let mut color_jeton = [1.0, 1.0, 1.0, 1.0];
                    match cell {
                        Jeton::NONE => {},
                        Jeton::YELLOW => {color_jeton = [1.0, 0.8, 0.0, 1.0];},
                        Jeton::RED => {color_jeton = [1.0, 0.0, 0.0, 1.0];}
                    }
                    ellipse(color_jeton, taille_trou, c.transform, g);
 
                }
            }

        });
    }
}
