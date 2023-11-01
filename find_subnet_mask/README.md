# <p align="center">Find subnet mask</p>

command line programming for find subnet mask

### Build

- `cargo build --release` 
- output file is `./target/release/find_subnet_mask`

### Run

- `file_subnet_mask <0-32>`
- `file_subnet_mask <0-32> <0-32> <0-32> <0-32>`


### example

```bash
$ find_subnet_mask 20
# /20 = 11111111.11111111.11110000.00000000 | 255.255.240.0
```

```bash
$ find_subnet_mask 10 20 30
# /10 = 11111111.11000000.00000000.00000000 | 255.192.0.0
# /20 = 11111111.11111111.11110000.00000000 | 255.255.240.0
# /30 = 11111111.11111111.11111111.11111100 | 255.255.255.252
```