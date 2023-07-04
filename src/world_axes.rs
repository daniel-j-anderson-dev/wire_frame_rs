use glam::DVec3;

pub struct WorldAxes {
    x_axis: DVec3,
    y_axis: DVec3,
    z_axis: DVec3
}

impl WorldAxes{
    pub fn new(x_basis: &DVec3, y_basis: &DVec3, z_basis: &DVec3) -> WorldAxes {
        return WorldAxes {
            x_axis: *x_basis,
            y_axis: *y_basis,
            z_axis: *z_basis
        }
    }
    pub fn default() -> WorldAxes {
        return WorldAxes {
            x_axis: (DVec3 { x: 1f64, y: 0f64, z: 0f64 }),
            y_axis: (DVec3 { x: 0f64, y: 1f64, z: 0f64 }),
            z_axis: (DVec3 { x: 0f64, y: 0f64, z: 1f64 })
        }
    }
    fn normalize_if_possible(self: &mut Self) {
        if self.x_axis.length() != 0f64 && self.x_axis.length() != 1f64 {
            self.x_axis = self.x_axis.normalize()
        }
        if self.y_axis.length() != 0f64 && self.y_axis.length() != 1f64 {
            self.y_axis = self.y_axis.normalize()
        }
        if self.z_axis.length() != 0f64 && self.y_axis.length() != 1f64 {
            self.z_axis = self.z_axis.normalize()
        }
    }
    pub fn rotate(self: &mut Self, rotation_axis: &DVec3, angle: &f64) {
        if rotation_axis.length() != 0f64 {
            let scaled_axis: glam::DVec3 = rotation_axis.normalize();
            let rotation: glam::DQuat = glam::DQuat::from_axis_angle(scaled_axis, *angle);
            self.x_axis = rotation.mul_vec3(self.x_axis);
            self.y_axis = rotation.mul_vec3(self.y_axis);
            self.z_axis = rotation.mul_vec3(self.z_axis);
        }
    }
    pub fn x(self: &Self) -> &DVec3 {
        return &(self.x_axis);
    }
    pub fn y(self: &Self) -> &DVec3 {
        return &(self.y_axis);
    }
    pub fn z(self: &Self) -> &DVec3 {
        return &(self.z_axis);
    }
    fn as_array(self: &Self) -> [DVec3; 3] {
        return [self.x_axis, self.y_axis, self.z_axis];
    }
    fn to_sdl_point_array(self: &mut Self, window_width: &f64, window_height: &f64) -> [sdl2::rect::Point; 3] {
        // TODO: add perspective projection
        self.normalize_if_possible();
        let size = 400f64;
        let sdl_points: [sdl2::rect::Point; 3] = [
            sdl2::rect::Point::new(
                (self.x_axis.x * size + window_width)  as i32,
                (self.x_axis.y * size + window_height) as i32
            ),
            sdl2::rect::Point::new(
                (self.y_axis.x * size + window_width)  as i32,
                (self.y_axis.y * size + window_height) as i32
            ),
            sdl2::rect::Point::new(
                (self.z_axis.x * size + window_width)  as i32,
                (self.z_axis.y * size + window_height) as i32
            )
        ];
        return sdl_points;
    }
    pub fn draw(self: &mut Self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(),String> {
        let start: sdl2::rect::Point = sdl2::rect::Point::new( // window origin
            (canvas.window().size().0 / 2u32) as i32,
            (canvas.window().size().1 / 2u32) as i32
        );
        let sdl_points: [sdl2::rect::Point; 3] = self.to_sdl_point_array(
            &(start.x as f64),
            &(start.y as f64)
        );

        let x_end: sdl2::rect::Point = sdl_points[0];
        let y_end: sdl2::rect::Point = sdl_points[1];
        let z_end: sdl2::rect::Point = sdl_points[2];
        
        canvas.set_draw_color(sdl2::pixels::Color::RED);
        canvas.draw_line(start, x_end)?;

        canvas.set_draw_color(sdl2::pixels::Color::GREEN);
        canvas.draw_line(start, y_end)?;

        canvas.set_draw_color(sdl2::pixels::Color::BLUE);
        canvas.draw_line(start, z_end)?;

        return Ok(());
    }
}