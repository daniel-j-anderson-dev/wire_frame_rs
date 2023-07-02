use glam::DVec3;

pub enum WorldAxis {
    X(glam::DVec3),
    Y(glam::DVec3),
    Z(glam::DVec3)
}

impl WorldAxis {
    fn rotate_dvec3(point_to_rotate: &mut glam::DVec3, rotation_axis: &glam::DVec3, angle: &f64) -> () {
        if rotation_axis.length() != 0f64 {
            let scaled_axis: glam::DVec3 = rotation_axis.normalize();
            let rotation: glam::DQuat = glam::DQuat::from_axis_angle(scaled_axis, *angle);
            *point_to_rotate = rotation.mul_vec3(*point_to_rotate);
        }
    }

    pub fn rotate(self: &mut Self, rotation_axis: &glam::DVec3, angle: &f64) {
        let (WorldAxis::X(orientation) | WorldAxis::Y(orientation) | WorldAxis::Z(orientation)) = self;
        WorldAxis::rotate_dvec3(orientation, rotation_axis, angle);
    }

    pub fn orientation(self: &Self) -> &DVec3 {
        let (WorldAxis::X(orientation) | WorldAxis::Y(orientation) | WorldAxis::Z(orientation)) = self;
        return orientation;
    }

    fn color(self: &Self) -> sdl2::pixels::Color {
        return match self {
            WorldAxis::X(_) => sdl2::pixels::Color::RED,
            WorldAxis::Z(_) => sdl2::pixels::Color::GREEN,
            WorldAxis::Y(_) => sdl2::pixels::Color::BLUE
        };
    }

    pub fn default_world_axes() -> [WorldAxis; 3] {
        return [
            WorldAxis::X(glam::DVec3::new(1.0, 0.0, 0.0)),
            WorldAxis::Y(glam::DVec3::new(0.0, 1.0, 0.0)),
            WorldAxis::Z(glam::DVec3::new(0.0, 0.0, 1.0))
        ];
    }

    fn to_sdl_point(self: &Self, window_width: &f64, window_height: &f64) -> sdl2::rect::Point {
        // TODO: add perspective projection
        let (WorldAxis::X(orientation) | WorldAxis::Y(orientation) | WorldAxis::Z(orientation)) = self;
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
