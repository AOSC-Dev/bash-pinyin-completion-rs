# bash-pinyin-completion-rs
Simple completion script for pinyin, written in rust. 

## Installation
**Clone the project**
```bash
git clone https://github.com/wxiwnd/bash-pinyin-completion-rs
cd bash-pinyin-completion-rs
```

**Build and Install**

Ensure that `bash-completion` and rust toolchains(cargo, etc.) are installed correctly.

```bash
bash scripts/install_completion.sh
```

And add these to your `.bashrc` file:
```bash
bind 'set show-all-if-ambiguous on'
bind 'TAB:menu-complete'
```

## Requirements
- bash-completion
- rust toolchains

## Bug report
If you encounter any issues, please report them on the GitHub issues page.

## License
This project is licensed under the GPLv3 License. See the [LICENSE](./LICENSE) file for details.