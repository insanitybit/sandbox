use ipc_channel::ipc;
use serde::{Serialize, Deserialize};
use libc::pid_t;
use nix::unistd::{Fork, fork, getpid, setuid, setgid};
use std::error::Error;
use sandbox_descriptor::SandboxDescriptor;

#[derive(Clone, Copy)]
pub enum UidOrGid {
    /// The nobody user - 65534
    Nobody,
    /// A user that does not exist. Defaults to Nobody if a user can not be selected.
    NonExistent,
    /// A specific user
    Distinct(u32)
}

impl From<u32> for UidOrGid {
    fn from(f: u32) -> UidOrGid {
        UidOrGid::Distinct(f)
    }
}

impl Into<u32> for UidOrGid {
    fn into(self) -> u32 {
        match self {
            UidOrGid::Nobody => 65534,
            UidOrGid::NonExistent => UidOrGid::Nobody.into(),
            UidOrGid::Distinct(u) => u
        }
    }
}

pub struct UnixUserGroupSandbox {
    uid: Option<UidOrGid>,
    gid: Option<UidOrGid>,
}

impl UnixUserGroupSandbox {
    pub fn new(uid: Option<UidOrGid>, gid: Option<UidOrGid>) -> UnixUserGroupSandbox {
        UnixUserGroupSandbox {
            uid: uid,
            gid: gid,
        }
    }
}

impl SandboxDescriptor for UnixUserGroupSandbox {
    fn execute(&mut self) -> Result<(), Box<Error>> {
        if let Some(gid) = self.gid {
            setgid(gid.into()).unwrap();
        };

        if let Some(uid) = self.uid {
            setuid(uid.into()).unwrap();
        };
        Ok(())
    }
}
