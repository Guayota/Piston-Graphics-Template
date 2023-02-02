extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::io::{Read, Write};

use glutin_window::GlutinWindow;
use graphics::Graphics;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

macro_rules! clear_term {
    () => {
        // Clear screen and render field at the top
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    };
}

macro_rules! print_flush {
    ($($t:tt)*) => {
        {
            write!(std::io::stdout(), $($t)*).unwrap();
            std::io::stdout().flush().unwrap();
        }
    }
}

macro_rules! println_flush {
    () => {
        println!();
        std::io::stdout().flush().unwrap();
    };
    ($($t:tt)*) => {
        {
            write!(std::io::stdout(), $($t)*).unwrap();
            println!();
            std::io::stdout().flush().unwrap();
        }
    }
}

const WIN_TITLE: &str  = "Title";
const WIN_WIDTH:  u32  = 480;
const WIN_HEIGHT: u32  = 480;

// Colors constants
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub trait ContentHandler {
    fn render_all(&mut self, args: &RenderArgs);
    fn update_all(&mut self);
    fn pressed(&mut self, btn_arg: &ButtonArgs);
}

pub trait Renderable {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs);
    fn update(&mut self);
}

type Items = Vec<Box<dyn Renderable>>;
struct Canvas {
    gl: GlGraphics,
    items: Items
}

impl ContentHandler for Canvas {
    fn render_all(&mut self, args: &RenderArgs) {
        // Background is cleared
        self.gl
            .draw(args.viewport(), |_c, gl| graphics::clear(BLACK, gl));

        // Update items
        self.items.iter()
            .for_each(|item| item.render(&mut self.gl, args));
    }

    fn update_all(&mut self) {
        self.items.iter_mut()
            .for_each(|item| item.update());
    }

    fn pressed(&mut self, btn_arg: &ButtonArgs) {
        match &btn_arg.button {
            &Button::Keyboard(Key::Space) => println_flush!("Space pressed!"),
            _ => ()
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow =
        WindowSettings::new(WIN_TITLE, [WIN_WIDTH as u32, WIN_HEIGHT as u32])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .vsync(true)
            .resizable(false) // Makes it so i3wm won't resize it
            .build()
            .unwrap();

    let mut content = Canvas {
        gl: GlGraphics::new(opengl),
        items: vec![]
    };

    let mut events = Events::new(EventSettings::new()).ups(60); // 60 FPS
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            content.render_all(&r);
        }

        if let Some(u) = e.update_args() {
            content.update_all();
        }

        if let Some(key) = e.button_args() {
            content.pressed(&key);
        }
    }
}
