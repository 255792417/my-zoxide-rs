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

4. Integrate with your shell
    Add the init command for your shell to the corresponding shell config file.

    **Fish** (`~/.config/fish/config.fish` on Linux):
    ```fish
    my-zoxide init fish | source
    ```

    **Bash** (`~/.bashrc`):
    ```bash
    eval "$(my-zoxide init bash)"
    ```

    **Zsh** (`~/.zshrc`):
    ```zsh
    eval "$(my-zoxide init zsh)"
    ```

    After reloading your shell config, you can use `my-z` for interactive directory jumping, and visited directories will be recorded automatically.

### How to uninstall  
just run these commands in your shell
```bash
my-zoxide clear
cargo uninstall my-zoxide
```