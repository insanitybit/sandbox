use std::error::Error;

pub trait SandboxDescriptor {
    fn execute(&mut self) -> Result<(), Box<Error>>;
}
