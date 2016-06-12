use ipc_channel::ipc;
use serde::{Serialize, Deserialize};
use nix::unistd::{Fork, fork};
use super::std;
use sandbox_descriptor::SandboxDescriptor;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SandboxResult<T> {
    Ok(T),
    Err,
}

impl<T> SandboxResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            SandboxResult::Ok(t) => t,
            _ => panic!(),
        }
    }
}

pub struct Sandbox {
    descriptors: Vec<Box<SandboxDescriptor>>,
}

impl Sandbox {
    pub fn new() -> Sandbox {
        Sandbox { descriptors: vec![] }
    }

    pub fn new_with_descriptors(descriptors: Vec<Box<SandboxDescriptor>>) -> Sandbox {
        Sandbox { descriptors: descriptors }
    }

    pub fn execute<F, T>(mut self, closure: F) -> SandboxResult<T>
        where F: Fn() -> T,
              T: Serialize + Deserialize
    {
        let (snd, rcv) = ipc::channel().unwrap();

        if let Fork::Parent(_) = fork().unwrap() {
            return match rcv.recv() {
                Ok(t) => SandboxResult::Ok(t),
                Err(_) => SandboxResult::Err,
            };
        }

        for descriptor in self.descriptors.iter_mut() {
            descriptor.execute().unwrap();
        }

        snd.send(closure()).unwrap();
        std::process::exit(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nix::unistd::{getuid, getgid};
    use unix_dac_sandbox::{UnixDacSandboxBuilder, UidOrGid};
    use std::path::Path;

    #[test]
    fn it_works() {
        let sandbox = Sandbox::new_with_descriptors(vec![
            // Create a Unix sandbox that will run code from the nobody user in a chroot with no
            // permissions
            Box::new(
                UnixDacSandboxBuilder::new()
                .with_uid(UidOrGid::Nobody)
                .with_gid(UidOrGid::Nobody)
                .with_chroot(Path::new("/run/sandbox/")) // create this beforehand
                .into_descriptor()
                )
            ]);

        let value = "yo";

        let result = sandbox.execute(|| {
            assert_eq!(getuid(), 65534);
            assert_eq!(getgid(), 65534);

            // Do scary things!
            format!("{}, I'm running in a sandbox", value) // capture variables
        });

        assert_eq!(result.unwrap(), "yo, I'm running in a sandbox");
    }
}
