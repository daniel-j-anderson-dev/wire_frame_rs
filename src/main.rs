use std::process::exit;

use sdl2::{
    EventPump,
    event::Event,
    keyboard::{Scancode, KeyboardState},
    gfx::framerate::FPSManager,
    rect::Point,
    pixels::Color, 
};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const WINDOW_TITLE: &str = "Wire Frames!";
const SPEED: i32 = 10;

fn main() -> Result<(), String> {
    // init sdl and subsystems
    let sdl = sdl2::init()?;
    let mut event_pump = sdl.event_pump()?;
    let video = sdl.video()?;
    let window = video.window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT).allow_highdpi().build().map_err(|err| err.to_string())?;
    let mut canvas = window.into_canvas().accelerated().build().map_err(|err| err.to_string())?;
    let mut framerate_manager = FPSManager::new();
    framerate_manager.set_framerate(60)?;

    // define points to draw a line between and their rates of change
    let mut a = Point::new(300, 400);
    let mut b = Point::new(500, 400);
    let mut a_vel = Point::new(0, 0);
    let mut b_vel = Point::new(0, 0);
    
    // main loop
    loop {
        handle_events(&mut event_pump);
        
        // figure out how to update state
        handle_input(&event_pump.keyboard_state(), &mut a_vel, &mut b_vel);
        
        // update state
        update_point(&mut a, &a_vel);
        update_point(&mut b, &b_vel);
        
        draw_line(&mut canvas, &a, &b);
        
        update_display(&mut canvas, &mut framerate_manager);
    }
}

// quits with 0 when esc is pressed or closed normaly with os
fn handle_events(event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown {scancode: Some(Scancode::Escape), ..} => {exit(0);}
            Event::KeyDown { .. } => {}
            Event::KeyUp { .. } => {}
            _ => {}
        }
    }
}

// Sets the velocity of the two points based on keyboard input
fn handle_input(keyboard_state: &KeyboardState, a_vel: &mut Point, b_vel: &mut Point) {
    // reset velocity from last frame
    a_vel.x = 0; a_vel.y = 0;
    b_vel.x = 0; b_vel.y = 0;
    
    // determin velocity for point a
    if keyboard_state.is_scancode_pressed(Scancode::W) {
        a_vel.y -= SPEED;
    }
    if keyboard_state.is_scancode_pressed(Scancode::A) {
        a_vel.x -= SPEED;
    }
    if keyboard_state.is_scancode_pressed(Scancode::S) {
        a_vel.y += SPEED;
    }
    if keyboard_state.is_scancode_pressed(Scancode::D) {
        a_vel.x += SPEED;
    }

    // determin velocity for point b
    if keyboard_state.is_scancode_pressed(Scancode::Up) {
        b_vel.y -= SPEED;
    }
    if keyboard_state.is_scancode_pressed(Scancode::Left) {
        b_vel.x -= SPEED;
    }
    if keyboard_state.is_scancode_pressed(Scancode::Down) {
        b_vel.y += SPEED;
    }
    if keyboard_state.is_scancode_pressed(Scancode::Right) {
        b_vel.x += SPEED;
    }       
}

// adds velocity to the point then ensures point is in bounds
fn update_point(point: &mut Point, velocity: &Point) {
    point.x += velocity.x;
    point.y += velocity.y;
    keep_point_in_window(point);
}

// moves point in bounds if its not
fn keep_point_in_window(point: &mut Point) {
    // left side
    if point.x < 0 {
        point.x = 0;
    }
    //right side
    if point.x > WINDOW_WIDTH as i32 {
        point.x = WINDOW_WIDTH as i32;
    }
    // top side
    if point.y < 0 {
        point.y = 0;
    }
    //bottom side
    if point.y > WINDOW_HEIGHT as i32 {
        point.y = WINDOW_HEIGHT as i32;
    }
}

// draws the a line between two points on the passed canvas
fn draw_line(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, start: &Point, end: &Point) {
    canvas.set_draw_color(Color::BLACK);
    canvas.draw_line(*start, *end).expect("couldn't draw line");        
}

// presents the canvas, resets it then waits for the next frame
fn update_display(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, framerate_manager: &mut FPSManager) {
    canvas.present();
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    framerate_manager.delay();
}