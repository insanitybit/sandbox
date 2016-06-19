[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/insanitybit/gsblookup-rs/blob/master/LICENSE-MIT)

# sandbox
Rust helpers for running functions in sandboxed environments.

# Usage

Not yet available on crates.io - unstable, unversioned, unverified.

# Notes On Sandbox Composition

This library allows you to build and stack sandbox descriptions and then build
the sandbox in one shot. This approach does not compose well - one sandbox may
require rights that a previous sandbox removed. Sandboxes may only partially
execute before returning an error.

To cope with this I highly suggest you pay attention to the order in which the
sandboxes are executed, and write tests for your sandboxes.

# Notes on performance

The cost of the sandbox includes:
* At least one fork (descriptors may incur their own forking)
* Serialization of function results - in order to return a value from the sandbox
it must be serialized.

# Notes on security

If we imagine that your sandbox is 'perfect' and the attacker has no access to
the system, then the path of least resistance is the broker process - the process
that creates the sandbox. In particular, the serialization library is directly
exposed to the attacker since they can potentially control return values.

I have not looked at the serialization library at all. I do not know if it uses
unsafe code.

# Example

```rust
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
```

In order to build you'll need to use a specific nightly, try:
```
  rustup override add nightly-2016-06-09
```
