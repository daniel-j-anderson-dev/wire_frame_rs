use std::process::exit;

use sdl2::{
    EventPump,
    event::Event,
    keyboard::{Scancode, KeyboardState},
    gfx::framerate::FPSManager,
    pixels::Color, 
};

use glam::f64::{
    DQuat,
    DVec3
};

// constants
const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;
const WINDOW_TITLE: &str = "Wire Frames!";
const SPEED: f64 = 10f64;
const ANGULAR_SPEED: f64 = 0.15f64;
const BG_COLOR: Color = Color::WHITE;
const DRAW_COLOR: Color = Color::BLACK;

struct Tri {
    verticies: [DVec3; 3]
}

impl Tri {
    fn rotate(self: &mut Self, rotation_axis: &DVec3, angle: &f64) -> () {
        if rotation_axis.length() != 0f64 {
            let scaled_axis = rotation_axis.normalize();
            let rotation: DQuat = DQuat::from_axis_angle(scaled_axis, *angle);
            for vertex in self.verticies.iter_mut() {
                *vertex = rotation.mul_vec3(*vertex);
            }
        }
    }
    fn translate(self: &mut Self, translation_axis: &DVec3, distance: &f64) -> () {
        if translation_axis.length() != 0f64 {
            let scaled_axis = translation_axis.normalize() * (*distance);
            for vertex in self.verticies.iter_mut() {
                *vertex += scaled_axis;
            }
        }
    }
    fn as_sdl_point_array(self: &Self) -> [sdl2::rect::Point; 3] {
        let mut sdl_points: [sdl2::rect::Point; 3] = [
            sdl2::rect::Point::new(0, 0),
            sdl2::rect::Point::new(0, 0),
            sdl2::rect::Point::new(0, 0)];
        for (i, vertex) in self.verticies.iter().enumerate() {
            sdl_points[i] = sdl2::rect::Point::new(
                (vertex.x + (WINDOW_WIDTH /2)as f64) as i32,
                (vertex.y + (WINDOW_HEIGHT/2)as f64) as i32
                // TODO: perspective projection.
            );
        }
        return sdl_points;
    }
    fn draw(self: &Self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        canvas.set_draw_color(DRAW_COLOR);
        let sdl_points = self.as_sdl_point_array();
        canvas.draw_line(sdl_points[0], sdl_points[1])?;
        canvas.draw_line(sdl_points[0], sdl_points[2])?;
        canvas.draw_line(sdl_points[1], sdl_points[2])?;
        return Ok(())
    }
    fn new(a: DVec3, b: DVec3, c: DVec3) -> Tri {
        return Tri { verticies: [a, b, c] }
    }
    fn default() -> Tri {
        const TRI_SIDE_LENGTH: f64 = (WINDOW_HEIGHT/2) as f64;
        const TRI_HEIGHT: f64 = TRI_SIDE_LENGTH * 0.8660254037844386f64;
        return Tri {
            verticies: [
                DVec3 { x: (0f64),                  y: (-TRI_SIDE_LENGTH/2f64), z: (0f64) },
                DVec3 { x: (-TRI_SIDE_LENGTH/2f64), y: (TRI_HEIGHT / 2f64),     z: (0f64) },
                DVec3 { x: ( TRI_SIDE_LENGTH/2f64), y: (TRI_HEIGHT / 2f64),     z: (0f64) }
            ]
        }
    }
}

fn main() -> Result<(), String> {
    // init sdl and subsystems
    let sdl = sdl2::init()?;
    let mut event_pump: EventPump = sdl.event_pump()?;
    let video = sdl.video()?;
    let window = video.window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT).allow_highdpi().build().map_err(|err| err.to_string())?;
    let mut canvas = window.into_canvas().accelerated().build().map_err(|err| err.to_string())?;
    let mut framerate_manager = FPSManager::new();
    framerate_manager.set_framerate(60)?;

    let mut triangle = Tri::default();

    let mut rotation_axis: DVec3 = DVec3:: new(0f64, 0f64, 0f64);
    let mut translation_axis: DVec3 = DVec3::new(0f64, 0f64, 0f64);

    // main loop
    loop {
        handle_events(&mut event_pump);
        
        // figure out how to update state
        handle_input(&event_pump.keyboard_state(), &mut triangle, &mut rotation_axis, &mut translation_axis);
        
        // update state
        triangle.rotate(&rotation_axis, &ANGULAR_SPEED);
        triangle.translate(&translation_axis, &SPEED);
        
        triangle.draw(&mut canvas)?;
        
        update_display(&mut canvas, &mut framerate_manager);
    }
}

// quits with 0 when esc is pressed or closed normaly with os
fn handle_events(event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown {scancode: Some(Scancode::Escape), ..} => {exit(0);}
            Event::KeyDown { scancode: Some(Scancode::Backspace), .. } => {}
            Event::KeyUp { .. } => {}
            _ => {}
        }
    }
}

// Sets the velocity of the two points based on keyboard input
fn handle_input(keyboard_state: &KeyboardState, triangle: &mut Tri, rotation_axis: &mut DVec3, translation_axis: &mut DVec3) {
    if keyboard_state.is_scancode_pressed(Scancode::F1) {
        *triangle = Tri::default();
    }
    // reset axes
    *rotation_axis = DVec3::new(0f64, 0f64, 0f64);
    *translation_axis = DVec3::new(0f64, 0f64, 0f64);
    
    // determine rotation axis
    if keyboard_state.is_scancode_pressed(Scancode::W) {
        rotation_axis.x -= 1f64/3f64; // deref not needed here since y belongs to rotation axis
    }
    if keyboard_state.is_scancode_pressed(Scancode::A) {
        rotation_axis.y -= 1f64/3f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::S) {
        rotation_axis.x += 1f64/3f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::D) {
        rotation_axis.y += 1f64/3f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::E) {
        rotation_axis.z += 1f64/3f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::Q) {
        rotation_axis.z -= 1f64/3f64;
    }

    // determine tanslation direction
    if keyboard_state.is_scancode_pressed(Scancode::Up) {
        translation_axis.y -= 1f64/3f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::Left) {
        translation_axis.x -= 1f64/3f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::Down) {
        translation_axis.y += 1f64/3f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::Right) {
        translation_axis.x += 1f64/3f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::PageUp) {
        translation_axis.z += 1f64/3f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::PageDown) {
        translation_axis.z -= 1f64/3f64;
    }       
}

// presents the canvas, resets it then waits for the next frame
fn update_display(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, framerate_manager: &mut FPSManager) {
    canvas.present();
    canvas.set_draw_color(BG_COLOR);
    canvas.clear();
    framerate_manager.delay();
}