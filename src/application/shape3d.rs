use glam::{DVec3, DQuat};
use sdl2::{render::Canvas, video::Window, rect::Point, pixels::Color};
use crate::application::Axes;
const PHI: f64 = 1.61803398874989484820;

#[derive(Debug)]
pub struct Shape3d {
    vertices: Vec<DVec3>,
    edges: Vec<[usize; 2]>,
    location: DVec3,
    local_axes: Axes,
    axes_hidden: bool
}

impl Shape3d {
    pub fn default() -> Self {
        let verticies: Vec<DVec3> = vec![];
        let edges: Vec<[usize; 2]> = vec![];
        return Self { 
            vertices: verticies,
            edges,
            location: DVec3::ZERO,
            local_axes: Axes::default(),
            axes_hidden: true
        }
    }
    pub fn new(vertices: Vec<DVec3>, edges: Vec<[usize; 2]>, location: DVec3) -> Self {
        return Self { vertices, edges, local_axes: Axes::new(DVec3::X, DVec3::Y, DVec3::Z, location), location, axes_hidden: true }
    }
    pub fn show_axes(&mut self) {
        self.axes_hidden = false;
    }
    pub fn hide_axes(&mut self) {
        self.axes_hidden = true;
    }
    pub fn location(&self) -> DVec3 {
        return DVec3 {
            x: self.location.x,
            y: self.location.y,
            z: self.location.z,
        }
    }
    pub fn vertices(&self) -> &Vec<DVec3> {
        return &self.vertices;
    }
    pub fn rotate(&mut self, rotation_center: &DVec3, rotation_axis: &DVec3, angle_radians: &f64) {
        if rotation_axis.length() != 0.0 {
            self.local_axes.rotate(rotation_center, rotation_axis, angle_radians);
            let rotation: DQuat = DQuat::from_axis_angle(*rotation_axis, *angle_radians);
            for vertex in self.vertices.iter_mut() {
                *vertex -= *rotation_center;
                *vertex  =  rotation.mul_vec3(*vertex);
                *vertex += *rotation_center;
            }
            self.location -= *rotation_center;
            self.location  =  rotation.mul_vec3(self.location);
            self.location += *rotation_center;
        }
    }
    pub fn translate(&mut self, translation_axis: &DVec3, distance: &f64) {
        if translation_axis.length() != 0.0 {
            self.local_axes.translate(translation_axis, distance);
            let delta_vertex = translation_axis.normalize() * (*distance);
            for vertex in self.vertices.iter_mut() {
                *vertex += delta_vertex;
            }
            self.location += delta_vertex;
        }
    }
    pub fn draw_orthographic(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        if !self.axes_hidden {
            self.local_axes.draw_orthographic(canvas, &100.0)?;
        }
        let [center_x, center_y] = [(canvas.window().size().0/2) as f64, (canvas.window().size().1/2) as f64];
        canvas.set_draw_color(Color::WHITE);
        for edge in self.edges.iter() {
            let vertex_a = self.vertices.get(edge[0]).unwrap();
            let vertex_b = self.vertices.get(edge[1]).unwrap();
            let start: Point = Point::new(
                (vertex_a.x + center_x) as i32,
                (vertex_a.y + center_y) as i32);
            let end:   Point = Point::new(
                (vertex_b.x + center_x) as i32,
                (vertex_b.y + center_y) as i32);
            canvas.draw_line(start, end)?;
        }
        return Ok(());
    }

    pub fn draw_weak_perspective(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        if !self.axes_hidden {
            self.local_axes.draw_orthographic(canvas, &100.0)?;
        }
        let [center_x, center_y] = [(canvas.window().size().0/2) as f64, (canvas.window().size().1/2) as f64];
        canvas.set_draw_color(Color::WHITE);
        for edge in self.edges.iter() {
            let mut vertex_a = *self.vertices.get(edge[0]).unwrap();
            let mut vertex_b = *self.vertices.get(edge[1]).unwrap();
            vertex_a *= vertex_a.distance(DVec3::ZERO) / vertex_a.z;
            vertex_b *= vertex_b.distance(DVec3::ZERO) / vertex_b.z;
            let start: Point = Point::new(
                (vertex_a.x + center_x) as i32,
                (vertex_a.y + center_y) as i32);
            let end:   Point = Point::new(
                (vertex_b.x + center_x) as i32,
                (vertex_b.y + center_y) as i32);
            canvas.draw_line(start, end)?;
        }
        return Ok(());
    }
}

pub fn cube(scale: f64, location: DVec3) -> Shape3d {
    let mut cube: Shape3d = Shape3d::new(
        vec![
            DVec3::new( scale,  scale,  scale),
            DVec3::new( scale,  scale, -scale),
            DVec3::new( scale, -scale,  scale),
            DVec3::new( scale, -scale, -scale),
            DVec3::new(-scale,  scale,  scale),
            DVec3::new(-scale,  scale, -scale),
            DVec3::new(-scale, -scale,  scale),
            DVec3::new(-scale, -scale, -scale)
        ],
        vec![
            [0, 1], [0, 2], [0, 4],
            [1, 3], [1, 5],
            [2, 3], [2, 6],
            [3, 7],
            [4, 5], [4, 6],
            [5, 7],
            [6, 7]
        ],
        location,
    );
    for vertex in cube.vertices.iter_mut() {
        *vertex += cube.location;
    }
    return cube;
}
pub fn tetrahedron(scale: f64, location: DVec3) -> Shape3d {
    let mut tetrahedron: Shape3d = Shape3d::new( 
        vec![
            DVec3::new( scale,  scale,  scale),  
            DVec3::new(-scale, -scale,  scale),
            DVec3::new(-scale,  scale, -scale),
            DVec3::new( scale, -scale, -scale)
        ],
        vec![
            [0, 1], [0, 2], [0, 3],
            [1, 2], [1, 3],
            [2, 3]
        ],
        location,
    );
    for vertex in tetrahedron.vertices.iter_mut() {
        *vertex += tetrahedron.location;
    }
    return tetrahedron;
}
pub fn octahedron(scale: f64, location: DVec3) -> Shape3d {
    let mut octahedron: Shape3d = Shape3d::new( 
        vec![
            DVec3::new( scale, 0.0,    0.0), 
            DVec3::new(-scale, 0.0,    0.0),
            DVec3::new( 0.0,   scale,  0.0),
            DVec3::new( 0.0,  -scale,  0.0),
            DVec3::new( 0.0,   0.0,    scale), 
            DVec3::new( 0.0,   0.0,   -scale)
        ],
        vec![
            [0, 2], [0, 3], [0, 4], [0, 5],
            [1, 2], [1, 3], [1, 4], [1, 5],
            [2, 4], [2, 5],
            [3, 4], [3, 5]
        ],
        location,
    );
    for vertex in octahedron.vertices.iter_mut() {
        *vertex += octahedron.location;
    }
    return octahedron;
}
pub fn dodecahedron(scale: f64, location: DVec3) -> Shape3d {
    let mut dodecahedron: Shape3d = Shape3d::new( 
        vec![
            DVec3::new( scale,      scale,      scale),      
            DVec3::new( scale,      scale,     -scale),      
            DVec3::new( scale,     -scale,      scale),       
            DVec3::new( scale,     -scale,     -scale),
            DVec3::new(-scale,      scale,      scale),     
            DVec3::new(-scale,      scale,     -scale),     
            DVec3::new(-scale,     -scale,      scale),      
            DVec3::new(-scale,     -scale,     -scale),
            DVec3::new( 0.0,        scale/PHI,  scale*PHI),
            DVec3::new( 0.0,        scale/PHI, -scale*PHI),
            DVec3::new( 0.0,       -scale/PHI,  scale*PHI),
            DVec3::new( 0.0,       -scale/PHI, -scale*PHI),
            DVec3::new( scale/PHI,  scale*PHI,  0.0),
            DVec3::new( scale/PHI, -scale*PHI,  0.0),
            DVec3::new(-scale/PHI,  scale*PHI,  0.0), 
            DVec3::new(-scale/PHI, -scale*PHI,  0.0),
            DVec3::new( scale*PHI,  0.0,        scale/PHI),
            DVec3::new( scale*PHI,  0.0,       -scale/PHI),
            DVec3::new(-scale*PHI,  0.0,        scale/PHI), 
            DVec3::new(-scale*PHI,  0.0,       -scale/PHI)
        ],
        vec![
            [0,  8], [0, 12], [ 0, 16], [ 1,  9], [ 1, 12], [ 1, 17],
            [2, 10], [2, 13], [ 2, 16], [ 3, 11], [ 3, 13], [ 3, 17],
            [4,  8], [4, 14], [ 4, 18], [ 5,  9], [ 5, 14], [ 5, 19],
            [6, 10], [6, 15], [ 6, 18], [ 7, 11], [ 7, 15], [ 7, 19],
            [8, 10], [9, 11], [12, 14], [13, 15], [16, 17], [18, 19]
        ],
        location,
    );
    for vertex in dodecahedron.vertices.iter_mut() {
        *vertex += dodecahedron.location;
    }
    return dodecahedron;
}
pub fn icosahedron(scale: f64, location: DVec3) -> Shape3d {
    let mut icosahedron: Shape3d = Shape3d::new( 
        vec![
            DVec3::new( 0.0,        scale,     scale*PHI),
            DVec3::new( 0.0,        scale,    -scale*PHI),
            DVec3::new( 0.0,       -scale,     scale*PHI),
            DVec3::new( 0.0,       -scale,    -scale*PHI),
            DVec3::new( scale,      scale*PHI, 0.0),
            DVec3::new( scale,     -scale*PHI, 0.0),
            DVec3::new(-scale,      scale*PHI, 0.0),
            DVec3::new(-scale,     -scale*PHI, 0.0),
            DVec3::new( scale*PHI,  0.0,       scale),
            DVec3::new( scale*PHI,  0.0,      -scale),
            DVec3::new(-scale*PHI,  0.0,       scale),
            DVec3::new(-scale*PHI,  0.0,      -scale)
        ],
        vec![
            [ 0,  2], [0,  4], [0, 6], [0,  8], [0, 10],
            [ 1,  3], [1,  4], [1, 6], [1,  9], [1, 11],
            [ 2,  5], [2,  7], [2, 8], [2, 10],
            [ 3,  5], [3,  7], [3, 9], [3, 11],
            [ 4,  6], [4,  8], [4, 9],
            [ 5,  7], [5,  8], [5, 9],
            [ 6, 10], [6, 11],
            [ 7, 10], [7, 11],
            [ 8,  9],
            [10, 11]
        ],
        location,
    );
    for vertex in icosahedron.vertices.iter_mut() {
        *vertex += icosahedron.location;
    }
    return icosahedron;
}  
pub fn platonic_solids(scale: f64) -> Vec<Shape3d> {
    return vec![
        crate::application::shape3d::cube(        scale,        DVec3 { x: 0.0,    y: 0.0,    z: 100.0 }),
        crate::application::shape3d::tetrahedron( scale,        DVec3 { x: 200.0,  y: 0.0,    z: 100.0 }),
        crate::application::shape3d::octahedron(  scale * 1.25, DVec3 { x:-200.0,  y: 0.0,    z: 100.0 }),
        crate::application::shape3d::dodecahedron(scale * 0.75, DVec3 { x: 0.0,    y: 200.0 , z: 100.0 }),
        crate::application::shape3d::icosahedron( scale * 0.75, DVec3 { x: 0.0,    y:-200.0,  z: 100.0 }),
    ]
}