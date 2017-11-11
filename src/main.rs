#![deny(missing_docs)]

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


/// Creates a maze using a popular algorithm
/// First element in return tuple is 2d vector
/// Second element in return tuple is array with first element 
/// corresponds to length of maze in x dimension.
/// Second element is length of maze in y dimension.
fn generate_maze() -> (Vec<Vec<u32>>, [usize; 2]) {
    (vec![vec![0, 0, 1, 1, 1, 1],
          vec![1, 0, 0, 0, 0, 1],
          vec![1, 1, 1, 1, 0, 1],
          vec![1, 1, 1, 1, 0, 1],
          vec![1, 1, 1, 1, 0, 1],
          vec![1, 1, 1, 1, 0, 0],
    ], [6, 6])
}

/// Hold datas of path already explored
struct MazeSolver {
}

impl MazeSolver {
    /// Implementation of Depth First Search
    fn dfs() {}

    /// Implementation of Breadth First Search
    fn bfs() {}

    /// Implementation of A* Search
    fn a_star() {}
}

/// Holds objects needed to create a graphics window
struct Screen {
    /// OpenGL drawing backend.
    gl: GlGraphics,
    /// Graphics Window
    window: Window,
    /// Graphics Events
    events: Events,
}

impl Screen {
    /// Create a new screen 
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

    /// Run the application indefinitely until player presses esc.
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

    /// Draws the maze on the graphics window.
    fn render(&mut self, args: &RenderArgs) {

        /// Arrays containing colors to be used
        const BLACK: [f32; 4]  = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4]  = [1.0, 1.0, 1.0, 1.0];
        const _RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const _GRAY: [f32; 4]  = [0.5, 0.5, 0.5, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen
            clear(WHITE, gl);

            let (maze, dim) = generate_maze(); 

            // buffer_h - buffer height, is the distance between the top of the screen and 
            // where the maze starts vertically.
            let buffer_h: f64 = (args.height as f64) / 10.0 / 2.0;

            // buffer_w - buffer width, is the distance between the left side of the screen and 
            // where the maze starts horizontally.
            let buffer_w: f64 = (args.width as f64)  / 10.0 / 2.0;

            // space_h - space height, is the amount of space each element in the maze vector take up vertically.
            let space_h: f64  = ((args.height as f64) - buffer_h * 2.0) / (dim[0] as f64);
            // space_w - space width, is the amount of space each element in the maze vector take up horizontally.
            let space_w: f64  = ((args.width as f64) -  buffer_w * 2.0) / (dim[1] as f64);
            
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

    /// Currently not used
    fn update(&mut self, _args: &UpdateArgs) {
        // Rotate 2 radians per second.

        // println!("{:?}", args);
    }
}

/// Start up maze.
fn main() {
    let mut screen = Screen::new("V3_2", 500, 500);
    screen.run();
}
