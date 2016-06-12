Easy sandboxing for rust code. Do not use yet, experimental, changing rapidly,
may end up unsupported.

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
