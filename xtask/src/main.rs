fn main() -> nih_plug_xtask::Result<()> {
    nih_plug_xtask::main()
}

/*
Lastly, create a `.cargo/config` file in your repository and add a Cargo alias.
This allows you to run the binary using `cargo xtask`:

```toml
# .cargo/config

[alias]
xtask = "run --package xtask --release --"
```

*/
