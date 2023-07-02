pub mod tri;
pub mod axis;
use std::process::exit;

use glam::f64::DVec3;
use sdl2::{
    EventPump, event::{Event, WindowEvent},
    keyboard::{Scancode,KeyboardState},
    pixels::Color
};

use tri::Tri;
use axis::Axis;

fn main() -> Result<(), String> {
    // init sdl and subsystems
    let sdl: sdl2::Sdl = sdl2::init()?;
    let mut event_pump: EventPump = sdl.event_pump()?;
    let video: sdl2::VideoSubsystem = sdl.video()?;
    let window: sdl2::video::Window = video.window("Wire Frames!", 800, 800)
        .allow_highdpi() // window settings 
        .resizable()
            .build() // create the window
            .map_err(|err| err.to_string())?; // make sure that worked
    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas()
        .accelerated() // canvas renderer settings
        .present_vsync()
            .build() // create the canvas
            .map_err(|err| err.to_string())?; // make sure that worked

    // init state variables
    let mut triangles: [Tri; 6] = Tri::cross();
    let mut world_axes: [Axis; 3] = Axis::default_world_axes();

    // init next state variables
    const SPEED: f64 = 10f64;
    const ANGULAR_SPEED: f64 = 0.15f64;
    const BG_COLOR: Color = Color::BLACK;
    let mut rotation_axis: DVec3 = DVec3:: new(0f64, 0f64, 0f64);
    let mut translation_axis: DVec3 = DVec3::new(0f64, 0f64, 0f64);
    let mut is_local_rotation: bool = true;
    let mut is_world_rotation: bool = false;
    let mut reset_requested: bool = false;

    // main loop
    loop {
        // prepair the canvas
        canvas.set_draw_color(BG_COLOR);
        canvas.clear();
        
        handle_events(&mut event_pump, &mut canvas)?;

        // sets the next state variables        
        handle_input(&event_pump.keyboard_state(), &mut reset_requested, &mut is_world_rotation, &mut is_local_rotation, &mut rotation_axis, &mut translation_axis);

        // update the current state variables based on next state variables
        if reset_requested {
            triangles = Tri::cross();
            world_axes = Axis::default_world_axes();
        };

        for triangle in triangles.iter_mut() {
            if is_world_rotation {
                triangle.rotate_global(&world_axes, &rotation_axis, &ANGULAR_SPEED);
                for axis in world_axes.iter_mut() {
                    axis.rotate(&rotation_axis, &(ANGULAR_SPEED/8f64));
                }
            }
            else if is_local_rotation {
                triangle.rotate_local(&rotation_axis, &ANGULAR_SPEED);
            }
            else {
                triangle.rotate_global(&world_axes, &rotation_axis, &ANGULAR_SPEED)
            }

            triangle.translate(&translation_axis, &SPEED);

            // draw each triangle
            triangle.draw(&mut canvas, Color::WHITE)?;
        }

        // draw world axes
        for axis in world_axes.iter_mut() {
            axis.draw(&mut canvas)?;
        }

        // display the canvas to the window
        canvas.present();
    }
}

fn handle_events(event_pump: &mut EventPump, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown {scancode: Some(Scancode::Escape), ..} => {
                exit(0);
            }
            Event::Window { win_event, .. } => {
                match win_event {
                    WindowEvent::Resized(width, height) => {
                        canvas.window_mut().set_size(width as u32, height as u32)
                        .map_err(|err| err.to_string())?;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    return Ok(());
}

fn handle_input(keyboard_state: &KeyboardState, reset_requested: &mut bool, is_world_rotation: &mut bool, is_local_rotation: &mut bool, rotation_axis: &mut DVec3, translation_axis: &mut DVec3) {
    // reset axes
    *rotation_axis = DVec3::new(0f64, 0f64, 0f64);
    *translation_axis = DVec3::new(0f64, 0f64, 0f64);
    *is_local_rotation = true;
    *is_world_rotation = false;
    *reset_requested = false;
    
    if keyboard_state.is_scancode_pressed(Scancode::F1) {
        *reset_requested = true;
    }
    if keyboard_state.is_scancode_pressed(Scancode::LShift) {
        *is_local_rotation = false;
    }
    if keyboard_state.is_scancode_pressed(Scancode::LCtrl) {
        *is_world_rotation = true;
    }

    // determine rotation axis
    if keyboard_state.is_scancode_pressed(Scancode::W) {
        rotation_axis.x -= 0.5773502691896257f64;
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