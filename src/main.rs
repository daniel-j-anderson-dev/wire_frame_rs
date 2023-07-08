pub mod application;
use crate::application::Application;

fn main() -> Result<(), String> {
    let mut wireframe_viewer = Application::new("Wireframe Mode")?;
    wireframe_viewer.run()?;
    return Ok(());
}