use nix::unistd::{setuid, setgid, chroot, chdir};
use std::error::Error;
use std::path::Path;
use sandbox_descriptor::{Platform, SandboxDescriptor};
use sandbox_error::SandboxError;

#[derive(Debug, Clone, Copy)]
pub enum UidOrGid {
    /// The nobody user - 65534
    Nobody,
    /// A specific user
    Distinct(u32),
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
            UidOrGid::Distinct(u) => u,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnixDacSandbox<'a> {
    uid: Option<UidOrGid>,
    gid: Option<UidOrGid>,
    chroot_dir: Option<&'a Path>,
}

#[derive(Debug, Clone)]
pub struct UnixDacSandboxBuilder<'a> {
    uid: Option<UidOrGid>,
    gid: Option<UidOrGid>,
    chroot_dir: Option<&'a Path>,
}

impl<'a> UnixDacSandboxBuilder<'a> {
    pub fn new() -> UnixDacSandboxBuilder<'a> {
        UnixDacSandboxBuilder {
            uid: None,
            gid: None,
            chroot_dir: None,
        }
    }

    pub fn with_uid(mut self, uid: UidOrGid) -> Self {
        self.uid = Some(uid);
        self
    }

    pub fn with_gid(mut self, gid: UidOrGid) -> Self {
        self.gid = Some(gid);
        self
    }

    pub fn get_uid(&self) -> Option<UidOrGid> {
        self.uid
    }

    pub fn get_gid(&self) -> Option<UidOrGid> {
        self.gid
    }

    pub fn with_chroot(mut self, chroot_dir: &'a Path) -> Self {
        self.chroot_dir = Some(chroot_dir);
        self
    }

    pub fn get_chroot(&self) -> Option<&'a Path> {
        self.chroot_dir
    }

    pub fn into_descriptor(self) -> UnixDacSandbox<'a> {
        UnixDacSandbox::from_builder(self)
    }
}

impl<'a> UnixDacSandbox<'a> {
    pub fn from_builder(builder: UnixDacSandboxBuilder<'a>) -> UnixDacSandbox<'a> {
        UnixDacSandbox {
            uid: builder.get_uid(),
            gid: builder.get_gid(),
            chroot_dir: builder.get_chroot(),
        }
    }
}

impl<'a> SandboxDescriptor for UnixDacSandbox<'a> {
    fn execute(&mut self) -> Result<(), SandboxError> {

        if let Some(chroot_dir) = self.chroot_dir {
            try!(chdir(chroot_dir));
            try!(chroot(chroot_dir));
        };

        if let Some(gid) = self.gid {
            try!(setgid(gid.into()));
        };

        if let Some(uid) = self.uid {
            try!(setuid(uid.into()));
        };
        Ok(())
    }

    fn get_platform_support(&self) -> Vec<Platform> {
        vec![Platform::Unix]
    }
}
