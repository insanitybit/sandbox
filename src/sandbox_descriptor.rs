use std::error::Error;

pub enum Platform {
    Unix,
    Windows,
    OSX,
}

pub trait SandboxDescriptor {
    fn execute(&mut self) -> Result<(), Box<Error>>;
    fn get_platform_support(&self) -> Vec<Platform>;
}
