### Running on Raspberry Pi

It'll be difficult to build on Raspberry Pi. A good way-around is to cross-build, then send the
executable to your Raspberry Pi.

First install the arm toolchain, for example on Ubuntu:

```bash
> sudo apt-get install gcc-arm-linux-gnueabihf
```

Then configure cargo to use the corresponding toolchain:

```bash
cat << EOF > ~/.cargo/config
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF
```

Finally, cross compile:

```bash
rustup target add armv7-unknown-linux-gnueabihf
cargo build --target=armv7-unknown-linux-gnueabihf
```
