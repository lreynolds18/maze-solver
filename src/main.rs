extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::glutin_window::GlutinWindow as Window;
use self::opengl_graphics::{ GlGraphics, OpenGL };
use self::graphics::*;

fn generate_maze() -> (Vec<Vec<u32>>, [usize; 2]) {
    (vec![vec![0, 0, 1, 1, 1, 1],
          vec![1, 0, 0, 0, 0, 1],
          vec![1, 1, 1, 1, 0, 1],
          vec![1, 1, 1, 1, 0, 1],
          vec![1, 1, 1, 1, 0, 1],
          vec![1, 1, 1, 1, 0, 0],
    ], [6, 6])
}

struct MazeSolver {
}

impl MazeSolver {
    fn dfs() {}

    fn bfs() {}

    fn a_star() {}
}

struct Screen {
    gl: GlGraphics,    // OpenGL drawing backend.
    window: Window,
    events: Events,
}

impl Screen {
    fn new(version: &str, width: u32, height: u32) -> Screen {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = match version {
            "V3_2" => OpenGL::V3_2,
            _      => OpenGL::V2_1
        };
            
        // Create an Glutin window.
        let window: Window = WindowSettings::new(
                "Maze Solver",
                [width, height]
            )
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let events = Events::new(EventSettings::new());

        Screen {
            gl : GlGraphics::new(opengl),
            window: window,
            events: events,
        }
    }

    fn run(&mut self) {
        while let Some(e) = self.events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }
    
            if let Some(u) = e.update_args() {
                self.update(&u);
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen
            clear(WHITE, gl);

            let (maze, dim) = generate_maze(); 

            // buffer is the space inbetween the max dimesion of the window and
            // where the maze starts and stops at
            // space inbetween top of the window and top of the maze
            // space inbetween the left side of the window and the left size of the maze
            // "" right, bottom
            let buffer_h: f64 = (args.height as f64) / 10.0 / 2.0;
            let buffer_w: f64 = (args.width as f64)  / 10.0 / 2.0;

            // space is the amount of space each element in the vector represents in the maze
            let space_h: f64  = ((args.height as f64) - buffer_h * 2.0) / (dim[0] as f64);
            let space_w: f64  = ((args.width as f64) -  buffer_w * 2.0) / (dim[1] as f64);

            println!("start width = {}", buffer_w);
            println!("total width = {}", ((args.width as f64) - buffer_w * 2.0));
            println!("space_w = {}", space_w);
            
            println!("transform = {:?}", c.transform);
            Rectangle::new(RED)
                .draw([buffer_w, buffer_h,
                       ((args.width as f64) - buffer_w * 2.0),
                       ((args.height as f64) - buffer_h * 2.0)
                      ],
                      &c.draw_state,
                      c.transform,
                      gl
                     );


            for j in 0..dim[1] {
                for i in 0..dim[0] {

                    let color = if maze[i][j] == 1 { BLACK } else { WHITE };

                    let x1 = buffer_w + space_w * (j as f64);
                    let y1 = buffer_h + space_h * (i as f64);

                    Rectangle::new(color)
                        .draw([x1, y1, space_w, space_h],
                              &c.draw_state,
                              c.transform,
                              gl
                             );
                }
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // Rotate 2 radians per second.

        // println!("{:?}", args);
    }
}

fn main() {
    let mut screen = Screen::new("V3_2", 500, 500);
    screen.run();
}
