pub enum Axis {
    X(glam::DVec3),
    Y(glam::DVec3),
    Z(glam::DVec3)
}

impl Axis {
    fn rotate_dvec3(point_to_rotate: &mut glam::DVec3, rotation_axis: &glam::DVec3, angle: &f64) -> () {
        if rotation_axis.length() != 0f64 {
            let scaled_axis: glam::DVec3 = rotation_axis.normalize();
            let rotation: glam::DQuat = glam::DQuat::from_axis_angle(scaled_axis, *angle);
            *point_to_rotate = rotation.mul_vec3(*point_to_rotate);
        }
    }

    pub fn rotate(self: &mut Self, rotation_axis: &glam::DVec3, angle: &f64) {
        let (Axis::X(orientation) | Axis::Y(orientation) | Axis::Z(orientation)) = self;
        Axis::rotate_dvec3(orientation, rotation_axis, angle);
    }

    fn color(self: &Self) -> sdl2::pixels::Color {
        return match self {
            Axis::X(_) => sdl2::pixels::Color::RED,
            Axis::Z(_) => sdl2::pixels::Color::GREEN,
            Axis::Y(_) => sdl2::pixels::Color::BLUE
        };
    }

    pub fn default_world_axes() -> [Axis; 3] {
        return [
            Axis::X(glam::DVec3::new(1.0, 0.0, 0.0)),
            Axis::Y(glam::DVec3::new(0.0, 1.0, 0.0)),
            Axis::Z(glam::DVec3::new(0.0, 0.0, 1.0))
        ];
    }

    fn to_sdl_point(self: &Self, window_width: &f64, window_height: &f64) -> sdl2::rect::Point {
        // TODO: add perspective projection
        let (Axis::X(orientation) | Axis::Y(orientation) | Axis::Z(orientation)) = self;
        return sdl2::rect::Point::new(
            ((orientation.x * 400f64) + window_width) as i32,
            ((orientation.y * 400f64) + window_height) as i32
        );
    }

    pub fn draw(self: &mut Self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        let start: sdl2::rect::Point = sdl2::rect::Point::new( // window origin
            (canvas.window().size().0 / 2u32) as i32,
            (canvas.window().size().1 / 2u32) as i32
        );
        let end: sdl2::rect::Point = self.to_sdl_point(
            &(start.x as f64),
            &(start.y as f64)
        );
        canvas.set_draw_color(self.color());
        canvas.draw_line(start, end)?;
        return Ok(());
    }
}
