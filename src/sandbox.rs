use ipc_channel::ipc;
use serde::{Serialize, Deserialize};
use nix::unistd::{Fork, fork};
use super::std;
use std::error::Error;
use sandbox_descriptor::SandboxDescriptor;
use sandbox_error::SandboxError;

#[derive(Serialize, Deserialize)]
enum Intern<T> {
    Success(T),
    Failure(String),
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

    pub fn execute<F, T>(mut self, closure: F) -> Result<T, SandboxError>
        where F: Fn() -> T,
              T: Serialize + Deserialize
    {
        let (snd, rcv) = try!(ipc::channel());

        let fork_res = try!(fork());

        if let Fork::Parent(_) = fork_res {
            return match rcv.recv() {
                Ok(t) => {
                    match t {
                        Intern::Success(t) => Ok(t),
                        Intern::Failure(e) => Err(SandboxError::SandboxFailure(e)),
                    }
                }
                Err(e) => Err(SandboxError::CommunicationError(e.description().to_owned())),
            };
        }

        for descriptor in self.descriptors.iter_mut() {
            if let Err(e) = descriptor.execute() {
                let mut err_str = match e.cause() {
                    Some(s) => format!("{} : {}", e.description(), s),
                    None => format!("{}", e.description()),
                };

                if let Some(f_str) = descriptor.fail_str() {
                    err_str = format!("{}: {}", f_str, err_str);
                }

                snd.send(Intern::Failure(err_str)).unwrap();
                std::process::exit(0);
            }
        }

        snd.send(Intern::Success(closure())).unwrap();
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
