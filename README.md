### About the project
A simple implementation of [zoxide](https://github.com/ajeetdsouza/zoxide)  

### Usage
1. Download the source code
```bash
git clone https://github.com/255792417/my-zoxide-rs my-zoxide
cd my-zoxide
```
2. Install with `cargo install`  
make sure you have already set up the toolchain for rust
```bash
cargo install --path .
```
3. Get Details  
you could get more details of this tool with:
```bash
my-zoxide --help
```
this will show a guide built with [clap](https://github.com/clap-rs/clap)

### How to uninstall  
just run these commands in your shell
```bash
my-zoxide clear
cargo uninstall my-zoxide
```