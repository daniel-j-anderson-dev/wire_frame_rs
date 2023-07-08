use glam::{DVec3, DQuat, DMat3};
use sdl2::{video::Window, render::Canvas, rect::Point, pixels::Color};

use crate::shape3d::Shape3d;

#[derive(Debug)]
pub struct Axes {
    x: DVec3,
    y: DVec3,
    z: DVec3,
    location: DVec3
}

impl Axes {
    pub fn new(x: DVec3, y: DVec3, z: DVec3, location: DVec3) -> Self {
        Self { x, y, z, location }
    }
    pub fn default() -> Self {
        Self { x: DVec3::X, y: DVec3::Y, z: DVec3::Z, location: DVec3::ZERO }
    }
    pub fn x(&self) -> &DVec3 {
        return &self.x;
    }
    pub fn y(&self) -> &DVec3 {
        return &self.y;
    }
    pub fn z(&self) -> &DVec3 {
        return &self.z;
    }
    pub fn location(&self) -> DVec3 {
       return self.location.clone(); 
    }
    pub fn rotate(&mut self, rotation_center: &DVec3, rotation_axis: &DVec3, angle_radians: &f64){
        if rotation_axis.length() != 0.0 {
            let rotation: DQuat = DQuat::from_axis_angle(*rotation_axis, *angle_radians);

            self.x -= *rotation_center;
            self.x  =  rotation.mul_vec3(self.x);
            self.x += *rotation_center;

            self.y -= *rotation_center;
            self.y  =  rotation.mul_vec3(self.y);
            self.y += *rotation_center;

            self.z -= *rotation_center;
            self.z  =  rotation.mul_vec3(self.z);
            self.z += *rotation_center;

            self.location -= *rotation_center;
            self.location = rotation.mul_vec3(self.location);
            self.location += *rotation_center;
        }
    }
    pub fn rotate_dquat(&mut self, rotation_center: &DVec3, rotation: &DQuat) {
        
        self.x -= *rotation_center;
        self.x  =  rotation.mul_vec3(self.x);
        self.x += *rotation_center;

        self.y -= *rotation_center;
        self.y  =  rotation.mul_vec3(self.y);
        self.y += *rotation_center;

        self.z -= *rotation_center;
        self.z  =  rotation.mul_vec3(self.z);
        self.z += *rotation_center;

        self.location -= *rotation_center;
        self.location = rotation.mul_vec3(self.location);
        self.location += *rotation_center;
    }
    pub fn translate(&mut self, translation_axis: &DVec3, distance: &f64) {
        if translation_axis.length() != 0.0 {
            let delta_vertex = translation_axis.normalize() * (*distance);
            self.x += delta_vertex;
            self.y += delta_vertex;
            self.z += delta_vertex;
            self.location += delta_vertex;
        }
    }
    pub fn draw_orthographic(&self, canvas: &mut Canvas<Window>, scale: &f64) -> Result<(), String> {
        let window_center = Point::new((canvas.window().size().0/2) as i32, (canvas.window().size().1/2) as i32);
        let start = Point::new(self.location.x as i32, self.location.y as i32) + window_center;
        let x_end = Point::new((self.x.x * scale) as i32, (self.x.y * scale) as i32) + start;
        let y_end = Point::new((self.y.x * scale) as i32, (self.y.y * scale) as i32) + start;
        let z_end = Point::new((self.z.x * scale) as i32, (self.z.y * scale) as i32) + start;
        
        canvas.set_draw_color(Color::RED);
        canvas.draw_line(start, x_end)?;        
        canvas.set_draw_color(Color::GREEN);
        canvas.draw_line(start, y_end)?;        
        canvas.set_draw_color(Color::BLUE);
        canvas.draw_line(start, z_end)?;
        canvas.draw_line(Point::new(0,0), Point::new(800,-800))?;    
        return Ok(());
    }
}