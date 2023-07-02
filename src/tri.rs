use crate::axis::Axis;
pub struct Tri {
    verticies: [glam::DVec3; 3],
    location: glam::DVec3
}

impl Tri {
    fn rotate_dvec3(point_to_rotate: &mut glam::DVec3, rotation_axis: &glam::DVec3, angle: &f64) -> () {
        if rotation_axis.length() != 0f64 {
            let scaled_axis: glam::DVec3 = rotation_axis.normalize();
            let rotation: glam::DQuat = glam::DQuat::from_axis_angle(scaled_axis, *angle);
            *point_to_rotate = rotation.mul_vec3(*point_to_rotate);
        }
    }

    pub fn rotate_global(self: &mut Self, world_axes: &[Axis; 3], rotation_axis: &glam::DVec3, angle: &f64) -> () {
        for vertex in self.verticies.iter_mut() {
            Tri::rotate_dvec3(vertex, rotation_axis, angle);
        }
        Tri::rotate_dvec3(&mut self.location, rotation_axis, angle);
    }

    pub fn rotate_local(self: &mut Self, rotation_axis: &glam::DVec3, angle: &f64) {
        let current_location = self.location;
        for vertex in self.verticies.iter_mut() {
            *vertex -= current_location;
            Tri::rotate_dvec3(vertex, rotation_axis, angle);
            *vertex += current_location;
        }
    }

    pub fn translate(self: &mut Self, translation_axis: &glam::DVec3, distance: &f64) -> () {
        if translation_axis.length() != 0f64 {
            let scaled_axis = translation_axis.normalize() * (*distance);
            for vertex in self.verticies.iter_mut() {
                *vertex += scaled_axis;
            }
            self.location += scaled_axis;
        }
    }

    fn to_sdl_point_array(self: &Self, window_width: &f64, window_height: &f64) -> [sdl2::rect::Point; 3] {
        let mut sdl_points: [sdl2::rect::Point; 3] = [
            sdl2::rect::Point::new(0, 0),
            sdl2::rect::Point::new(0, 0),
            sdl2::rect::Point::new(0, 0)];
        for (i, vertex) in self.verticies.iter().enumerate() {
            sdl_points[i] = sdl2::rect::Point::new(
                (vertex.x + window_width / 2f64) as i32,
                (vertex.y + window_height / 2f64) as i32
                // TODO: perspective projection.
            );
        }
        return sdl_points;
    }

    pub fn draw(self: &Self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, color: sdl2::pixels::Color) -> Result<(), String> {
        canvas.set_draw_color(color);
        let sdl_points: [sdl2::rect::Point; 3] = self.to_sdl_point_array(
            &(canvas.window().size().0 as f64),
            &(canvas.window().size().1 as f64)
        );
        canvas.draw_line(sdl_points[0], sdl_points[1])?;
        canvas.draw_line(sdl_points[0], sdl_points[2])?;
        canvas.draw_line(sdl_points[1], sdl_points[2])?;
        return Ok(())
    }

    fn default() -> Tri {
        const TRI_SIDE_LENGTH: f64 = 100f64;
        const TRI_HEIGHT: f64 = TRI_SIDE_LENGTH * 0.8660254037844386f64;
        let mut default = Tri {
            verticies: [
                glam::DVec3 { x: (0f64),                  y: (0f64),        z: (0f64) },
                glam::DVec3 { x: (-TRI_SIDE_LENGTH/2f64), y: (-TRI_HEIGHT), z: (0f64) },
                glam::DVec3 { x: ( TRI_SIDE_LENGTH/2f64), y: (-TRI_HEIGHT), z: (0f64) }
            ],
            location: glam::DVec3 { x: (0f64), y: (0f64), z: (0f64) }
        };
        for vertex in default.verticies.iter_mut() {
            *vertex += default.location;
        }
        return default;
    }

    pub fn cross() -> [Tri; 6] {
        return [Tri::top_tri(), Tri::bottom_tri(), Tri::left_tri(), Tri::right_tri(), Tri::front_tri(), Tri::back_tri()];
    }

    fn top_tri() -> Tri {
        return Tri::default();
    }

    fn bottom_tri() -> Tri {
        let mut bottom_tri = Tri::default();
        bottom_tri.rotate_local(&glam::DVec3::Z, &3.1415926535897932f64);
        return bottom_tri;
    }

    fn left_tri() -> Tri {
        let mut left_tri = Tri::default();
        left_tri.rotate_local(&glam::DVec3::Z, &(3.1415926535897932f64/2f64));
        return left_tri;
    }

    fn right_tri() -> Tri {
        let mut right_tri = Tri::default();
        right_tri.rotate_local(&glam::DVec3::Z, &(-3.1415926535897932f64/2f64));
        return right_tri;
    }

    fn front_tri() -> Tri {
        let mut front_tri = Tri::default();
        front_tri.rotate_local(&glam::DVec3::X, &(-3.1415926535897932f64/2f64));
        return front_tri;
    }

    fn back_tri() -> Tri {
        let mut back_tri = Tri::default();
        back_tri.rotate_local(&glam::DVec3::X, &(3.1415926535897932f64/2f64));
        return back_tri;
    }
}