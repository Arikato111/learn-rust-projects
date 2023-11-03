# <p align="center">Find subnet mask</p>

command line programming for find subnet mask

### Install (For linux)

```bash
curl -o- https://raw.githubusercontent.com/Arikato111/learn-rust-projects/main/find_subnet_mask/release/install.sh | bash
```

```bash
wget -qO- https://raw.githubusercontent.com/Arikato111/learn-rust-projects/main/find_subnet_mask/release/install.sh | bash
```

### Build

- `cargo build --release`
- output file is `./target/release/find_subnet_mask`

### Run

- `subnet <0-32>`
- `subnet <0-32> <0-32> <0-32> <0-32>`

### example

```bash
$ subnet 20
# /20 = 11111111.11111111.11110000.00000000 | 255.255.240.0
```

```bash
$ subnet 10 20 30
# /10 = 11111111.11000000.00000000.00000000 | 255.192.0.0
# /20 = 11111111.11111111.11110000.00000000 | 255.255.240.0
# /30 = 11111111.11111111.11111111.11111100 | 255.255.255.252
```
