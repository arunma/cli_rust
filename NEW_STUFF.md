### Creating a workspace

*Reference:* https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

```
cargo new cli_rust

```

#### Modify Cargo.toml

```
[workspace]
members=[
    "echor",
    "catr"
]
```

### Add new member to the workspace

```
cargo new echor
cargo new catr
```



### Run and test

```
cargo run --bin echor -- "hello world"
cargo test -p catr
```

### Add dependency at a crate level

```
cargo add rand -p catr    

```

