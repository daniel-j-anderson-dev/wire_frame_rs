
pub mod shape3d;
pub mod axes;

use sdl2::{keyboard::Scancode, event::{Event, WindowEvent}, pixels::Color};
use glam::DVec3;

use crate::application::{shape3d::Shape3d, axes::Axes};


#[derive(Debug)]
enum Rotation {
    Local, // represents rotations that rotate shapes relative to their repective locations
    Global, // represents rotations that rotate shapes reative to the world origin defined by world_axes
    CoordSystem, // will rotate the world_axes along with Local shape rotation. world_axes will also be translated
}

pub struct Application {
    event_pump: sdl2::EventPump,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    active: bool,

    // current state
    world_axes: Axes,
    shapes: Vec<Shape3d>,

    // next state
    rotation_center: DVec3,
    rotation_axis: DVec3,
    translation_axis: DVec3,
    delta_angle: f64,
    delta_location: f64,

    // flags
    rotation_type: Rotation, // types of rotations on the shapes are local, global
    shape_axes_hidden: bool,
}

impl Application {
    pub fn new(title: &str) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let event_pump = sdl.event_pump()?;
        let video_subsystem = sdl.video()?;
        let window = video_subsystem.window(title, 800, 800)
            .allow_highdpi().resizable().build().map_err(|e|e.to_string())?;
        let canvas = window.into_canvas()
            .accelerated().present_vsync().build().map_err(|e|e.to_string())?;

        return Ok(Self {
            event_pump,
            canvas,
            active: true,
            shapes: shape3d::platonic_solids(50.0),
            world_axes: Axes::default(),
            rotation_center: DVec3::ZERO, 
            rotation_axis: DVec3::ZERO,
            translation_axis: DVec3::ZERO,
            delta_angle: 0.05,
            delta_location: 5.0,
            rotation_type: Rotation::Local,
            shape_axes_hidden: true,
        });
    }
  
    fn handle_events(&mut self) -> Result<(), String>{
        self.handle_input();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
                    self.active = false;
                }
                Event::KeyDown { scancode: Some(scancode), .. } => {
                    match scancode {
                        Scancode::F1 => {
                            self.shapes = shape3d::platonic_solids(50.0);
                            self.world_axes = Axes::default();
                            println!("RESET!");
                        }
                        Scancode::F2 => {
                            match self.rotation_type {
                                Rotation::Local => {},
                                _ => {
                                    self.rotation_type = Rotation::Local;
                                    println!("Local Transformations");
                                }
                            }
                        }
                        Scancode::F3 => {
                            match self.rotation_type {
                                Rotation::Global => {},
                                _ => {
                                    self.rotation_type = Rotation::Global;
                                    println!("Global Transformations");
                                }
                            }
                        }
                        Scancode::F4 => {
                            match self.rotation_type {
                                Rotation::CoordSystem => {}, 
                                _ => {
                                    self.rotation_type = Rotation::CoordSystem;
                                    println!("Coordinate System Transformations");
                                }
                            }
                        }
                        Scancode::F5 => {
                            self.shape_axes_hidden = !self.shape_axes_hidden;
                            println!("{}", if self.shape_axes_hidden {"Show Shape Axes"} else {"Hide shape Axes"});
                        }
                        _ => {}
                    }
                }
                Event::Window { win_event, .. } => {
                    match win_event { 
                        WindowEvent::Resized(width, height) => {
                            self.canvas.window_mut().set_size(width as u32, height as u32)
                                .map_err(|err| err.to_string())?;
                        }
                        _ => {}
                    }
                }
                // other events
                _ => {}
            }
        }
        return Ok(());
    }
  
    fn handle_input(&mut self) {
        self.rotation_axis = DVec3::ZERO;
        self.translation_axis = DVec3::ZERO;
        let keys = self.event_pump.keyboard_state();

        if keys.is_scancode_pressed(Scancode::W) {
            self.rotation_axis += *self.world_axes.x();
        }
        if keys.is_scancode_pressed(Scancode::A) {
            self.rotation_axis -= *self.world_axes.y();
        }
        if keys.is_scancode_pressed(Scancode::S) {
            self.rotation_axis -= *self.world_axes.x();
        }
        if keys.is_scancode_pressed(Scancode::D) {
            self.rotation_axis += *self.world_axes.y();
        }
        if keys.is_scancode_pressed(Scancode::Q) {
            self.rotation_axis -= *self.world_axes.z();
        }
        if keys.is_scancode_pressed(Scancode::E) {
            self.rotation_axis += *self.world_axes.z();
        }

        if keys.is_scancode_pressed(Scancode::Up) {
            self.translation_axis -= *self.world_axes.y();
        }
        if keys.is_scancode_pressed(Scancode::Down) {
            self.translation_axis += *self.world_axes.y();
        }
        if keys.is_scancode_pressed(Scancode::Left) {
            self.translation_axis -= *self.world_axes.x();
        }
        if keys.is_scancode_pressed(Scancode::Right) {
            self.translation_axis += *self.world_axes.x();
        }
        if keys.is_scancode_pressed(Scancode::PageUp) {
            self.translation_axis -= *self.world_axes.z();
        }
        if keys.is_scancode_pressed(Scancode::PageDown) {
            self.translation_axis += *self.world_axes.z();
        }

        if self.rotation_axis.length() != 0.0 && self.rotation_axis.length() != 1.0 {
            self.rotation_axis = self.rotation_axis.normalize();
        }
        if self.rotation_axis.length() != 0.0 && self.rotation_axis.length() != 1.0 {
            self.rotation_axis = self.rotation_axis.normalize();
        }
    }
    
    fn update_canvas(&mut self) {
        self.canvas.present();
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }
    
    fn update_state(&mut self) -> Result<(), String> {
        for shape in self.shapes.iter_mut() {
            if self.shape_axes_hidden {
                shape.show_axes();
            } else {
                shape.hide_axes();
            }
            match self.rotation_type {
                Rotation::Local => {
                    self.rotation_center = shape.location();
                }
                Rotation::Global => {
                    self.rotation_center = self.world_axes.location();
                }
                Rotation::CoordSystem => {
                    self.rotation_center = self.world_axes.location();
                }
            }
            shape.rotate(&self.rotation_center, &self.rotation_axis, &self.delta_angle);
            shape.translate(&self.translation_axis, &self.delta_location);
            shape.draw_orthographic(&mut self.canvas)?;
        }
        match self.rotation_type {
            Rotation::CoordSystem => {
                self.world_axes.rotate(&self.rotation_center, &self.rotation_axis, &self.delta_angle);
                self.world_axes.translate(&self.translation_axis, &self.delta_location);
            }
            _ => {}
        }
        self.world_axes.draw_orthographic(&mut self.canvas, &400.0)?;
        return Ok(());
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.active {
            self.handle_events()?;
            
            self.update_state()?;

            self.update_canvas();
        }
        return Ok(());
    }
}