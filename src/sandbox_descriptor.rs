use sandbox_error::SandboxError;

pub enum Platform {
    Unix,
    Windows,
    OSX,
}

pub trait SandboxDescriptor {
    fn execute(&mut self) -> Result<(), SandboxError>;
    fn get_platform_support(&self) -> Vec<Platform>;
    fn fail_str(&self) -> Option<String>;
}
