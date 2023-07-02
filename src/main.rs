use std::{process::exit};

use glam::f64::{DQuat, DVec3};
use sdl2::{
    EventPump,
    event::Event,
    keyboard::{Scancode,KeyboardState},
    pixels::Color, 
};

// constants
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const WINDOW_TITLE: &str = "Wire Frames!";
const SPEED: f64 = 10f64;
const ANGULAR_SPEED: f64 = 0.15f64;
const BG_COLOR: Color = Color::WHITE;

fn main() -> Result<(), String> {
    // init sdl and subsystems
    let sdl = sdl2::init()?;
    let mut event_pump: EventPump = sdl.event_pump()?;
    let video = sdl.video()?;
    let window = video.window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT).allow_highdpi().build().map_err(|err| err.to_string())?;
    let mut canvas = window.into_canvas().accelerated().present_vsync().build().map_err(|err| err.to_string())?;

    let mut triangles: [Tri; 6] = Tri::cross();

    let colors: [Color; 6] = [Color::RED, Color::BLUE, Color::GREEN, Color::CYAN, Color::MAGENTA, Color::YELLOW];

    let mut rotation_axis: DVec3 = DVec3:: new(0f64, 0f64, 0f64);
    let mut translation_axis: DVec3 = DVec3::new(0f64, 0f64, 0f64);
    let mut is_local_rotation: bool = true;

    // main loop
    loop {
        canvas.set_draw_color(BG_COLOR);
        canvas.clear();
        
        handle_events(&mut event_pump);
        
        handle_input(&event_pump.keyboard_state(), &mut triangles, &mut is_local_rotation, &mut rotation_axis, &mut translation_axis);
        for (i, triangle) in triangles.iter_mut().enumerate() {
            match is_local_rotation {
                true  => triangle.rotate_local(&rotation_axis, &ANGULAR_SPEED),
                false => triangle.rotate_global(&rotation_axis, &ANGULAR_SPEED)
            }
            triangle.translate(&translation_axis, &SPEED);
            triangle.draw(&mut canvas, colors[i])?;
        }

        canvas.present();
    }
}

struct Tri {
    verticies: [DVec3; 3],
    location: DVec3
}

impl Tri {
    fn rotate_dvec3(point_to_rotate: &mut DVec3, rotation_axis: &DVec3, angle: &f64) -> () {
        if rotation_axis.length() != 0f64 {
            let scaled_axis = rotation_axis.normalize();
            let rotation: DQuat = DQuat::from_axis_angle(scaled_axis, *angle);
            *point_to_rotate = rotation.mul_vec3(*point_to_rotate);
        }
    }
    fn rotate_global(self: &mut Self, rotation_axis: &DVec3, angle: &f64) -> () {
        for vertex in self.verticies.iter_mut() {
            Tri::rotate_dvec3(vertex, rotation_axis, angle);
        }
        Tri::rotate_dvec3(&mut self.location, rotation_axis, angle);
    }
    fn rotate_local(self: &mut Self, rotation_axis: &DVec3, angle: &f64) {
        let current_location = self.location;
        for vertex in self.verticies.iter_mut() {
            *vertex -= current_location;
            Tri::rotate_dvec3(vertex, rotation_axis, angle);
            *vertex += current_location;
        }
    }
    fn translate(self: &mut Self, translation_axis: &DVec3, distance: &f64) -> () {
        if translation_axis.length() != 0f64 {
            let scaled_axis = translation_axis.normalize() * (*distance);
            for vertex in self.verticies.iter_mut() {
                *vertex += scaled_axis;
            }
            self.location += scaled_axis;
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
    fn draw(self: &Self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, color: Color) -> Result<(), String> {
        canvas.set_draw_color(color);
        let sdl_points = self.as_sdl_point_array();
        canvas.draw_line(sdl_points[0], sdl_points[1])?;
        canvas.draw_line(sdl_points[0], sdl_points[2])?;
        canvas.draw_line(sdl_points[1], sdl_points[2])?;
        return Ok(())
    }
    fn new(a: DVec3, b: DVec3, c: DVec3, offset: DVec3) -> Tri {
        return Tri { verticies: [a+offset, b+offset, c+offset], location: offset}
    }
    fn default() -> Tri {
        const TRI_SIDE_LENGTH: f64 = 100f64;
        const TRI_HEIGHT: f64 = TRI_SIDE_LENGTH * 0.8660254037844386f64;
        let mut default = Tri {
            verticies: [
                DVec3 { x: (0f64),                  y: (0f64),        z: (0f64) },
                DVec3 { x: (-TRI_SIDE_LENGTH/2f64), y: (-TRI_HEIGHT), z: (0f64) },
                DVec3 { x: ( TRI_SIDE_LENGTH/2f64), y: (-TRI_HEIGHT), z: (0f64) }
            ],
            location: DVec3 { x: (0f64), y: (0f64), z: (-100f64) }
        };
        for vertex in default.verticies.iter_mut() {
            *vertex += default.location;
        }
        return default;
    }
    fn top_tri() -> Tri {
        return Tri::default();
    }
    fn bottom_tri() -> Tri {
        let mut bottom_tri = Tri::default();
        bottom_tri.rotate_local(&DVec3::Z, &3.1415926535897932f64);
        return bottom_tri;
    }
    fn left_tri() -> Tri {
        let mut left_tri = Tri::default();
        left_tri.rotate_local(&DVec3::Z, &(3.1415926535897932f64/2f64));
        return left_tri;
    }
    fn right_tri() -> Tri {
        let mut right_tri = Tri::default();
        right_tri.rotate_local(&DVec3::Z, &(-3.1415926535897932f64/2f64));
        return right_tri;
    }
    fn front_tri() -> Tri {
        let mut front_tri = Tri::default();
        front_tri.rotate_local(&DVec3::X, &(-3.1415926535897932f64/2f64));
        return front_tri;
    }
    fn back_tri() -> Tri {
        let mut back_tri = Tri::default();
        back_tri.rotate_local(&DVec3::X, &(3.1415926535897932f64/2f64));
        return back_tri;
    }
    fn cross() -> [Tri; 6] {
        return [Tri::top_tri(), Tri::bottom_tri(), Tri::left_tri(), Tri::right_tri(), Tri::front_tri(), Tri::back_tri()];
    }
}

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

fn handle_input(keyboard_state: &KeyboardState, triangles: &mut [Tri; 6], is_local_rotation: &mut bool, rotation_axis: &mut DVec3, translation_axis: &mut DVec3) {
    if keyboard_state.is_scancode_pressed(Scancode::F1) {
        *triangles = Tri::cross(); // deref needed here
    }
    // reset axes
    *rotation_axis = DVec3::new(0f64, 0f64, 0f64);
    *translation_axis = DVec3::new(0f64, 0f64, 0f64);
    *is_local_rotation = true;
    
    if keyboard_state.is_scancode_pressed(Scancode::LShift) {
        *is_local_rotation = false;
    }

    // determine rotation axis
    if keyboard_state.is_scancode_pressed(Scancode::W) {
        rotation_axis.x -= 0.5773502691896257f64; // but not needed here? why?????????
    }
    if keyboard_state.is_scancode_pressed(Scancode::A) {
        rotation_axis.y -= 0.5773502691896257f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::S) {
        rotation_axis.x += 0.5773502691896257f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::D) {
        rotation_axis.y += 0.5773502691896257f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::E) {
        rotation_axis.z += 0.5773502691896257f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::Q) {
        rotation_axis.z -= 0.5773502691896257f64;
    }

    // determine tanslation direction
    if keyboard_state.is_scancode_pressed(Scancode::Up) {
        translation_axis.y -= 0.5773502691896257f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::Left) {
        translation_axis.x -= 0.5773502691896257f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::Down) {
        translation_axis.y += 0.5773502691896257f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::Right) {
        translation_axis.x += 0.5773502691896257f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::PageUp) {
        translation_axis.z += 0.5773502691896257f64;
    }
    if keyboard_state.is_scancode_pressed(Scancode::PageDown) {
        translation_axis.z -= 0.5773502691896257f64;
    }       
}