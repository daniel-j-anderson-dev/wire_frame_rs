pub mod application;
pub mod shape3d;
pub mod axes;
use crate::application::Application;

fn main() -> Result<(), String> {
    let mut wireframe_viewer = Application::new("Wireframe Mode")?;
    wireframe_viewer.run()?;
    return Ok(());
}