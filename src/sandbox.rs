use ipc_channel::ipc;
use serde::{Serialize, Deserialize};
use libc::pid_t;
use nix::unistd::{Fork, fork, getpid};
use super::std;
use std::error::Error;
use sandbox_descriptor::SandboxDescriptor;
use unix_dac_sandbox::UnixUserGroupSandbox;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SandboxResult<T> {
    Ok(T),
    Err
}

impl<T> SandboxResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            SandboxResult::Ok(t)    => t,
            _   => panic!()
        }
    }
}

pub struct Sandbox {
    executors: Vec<Box<SandboxDescriptor>>
}

impl Sandbox {
    pub fn new() -> Sandbox {
        Sandbox {
            executors: vec![]
        }
    }

    pub fn new_with_executors(executors: Vec<Box<SandboxDescriptor>>) -> Sandbox {
        Sandbox {
            executors: executors
        }
    }

    pub fn sandbox<F, T>(mut self, closure: F) -> SandboxResult<T>
        where F : Fn() -> T,
        T: Serialize + Deserialize
    {
        let (snd, rcv) = ipc::channel().unwrap();

        if let Fork::Parent(child_pid) = fork().unwrap() {
            return match rcv.recv() {
                Ok(t)   => SandboxResult::Ok(t),
                Err(_)  => SandboxResult::Err
            };
        }

        for descriptor in self.executors.iter_mut() {
            descriptor.execute();
        }

        snd.send(closure()).unwrap();
        std::process::exit(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nix::unistd::{getpid, getuid, getgid};
    use ::unix_dac_sandbox::{UnixUserGroupSandbox, UidOrGid};

    #[test]
    fn it_works() {
        let sandbox = Sandbox::new_with_executors(vec![
            Box::new(UnixUserGroupSandbox::new(Some(UidOrGid::Nobody),Some(UidOrGid::Nobody)))
            ]);

        let value = "yo";

        let result = sandbox.sandbox(|| {
            assert_eq!(getuid(), 65534);
            assert_eq!(getgid(), 65534);
            // Do scary things!
            unsafe {
                format!("{}, I'm running in a sandbox", value) // capture variables
            }
        });

        assert_eq!(result.unwrap(), "yo, I'm running in a sandbox");
    }
}
