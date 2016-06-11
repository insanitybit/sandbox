Easy sandboxing for rust code. Do not use yet, experimental, changing rapidly,
may end up unsupported.

```rust
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
```

In order to build you'll need to use a specific nightly, try:
```
  rustup override add nightly-2016-06-09
```
