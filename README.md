# Sample key remapper using [kiri](https://github.com/nanikamado/kiri)

## How to config

See the main function of [src/main.rs](src/main.rs).

## Installation

1. [Install Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html).
2. `git clone 'https://github.com/nanikamado/kiri-example-remapper'`
3. `cd kiri-example-remapper`
4. `cargo install --path .`

## Run
```sh
sudo kiri-example-remapper
```

or

```sh
cargo build
sudo target/debug/kiri-example-remapper
```

or 

```sh
cargo build
sudo env RUST_LOG=trace target/debug/kiri
```

for debugging


## Run automatically on startup (tested only in Fedora 36)

```sh
cd kiri-example-remapper
cargo b -r
sudo cp target/release/kiri-example-remapper /usr/bin/hoge-hoge-remapper
echo '[Unit]
Description=hoge-hoge-remapper

[Service]
ExecStart=/usr/bin/hoge-hoge-remapper
Type=simple

[Install]
WantedBy = multi-user.target
' > /etc/systemd/system/hoge-hoge-remapper.service
sudo systemctl enable hoge-hoge-remapper
sudo systemctl start hoge-hoge-remapper
```



