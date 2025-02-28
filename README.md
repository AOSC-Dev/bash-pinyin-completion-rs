# bash-pinyin-completion-rs
Completion script for pinyin, matcher based on [IbPinyinLib](https://github.com/Chaoses-Ib/IbPinyinLib)

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

For better experience, add these to your inputrc (/etc/inputrc, ~/.inputrc):
```
set show-all-if-ambiguous on
set menu-complete-display-prefix on
TAB: menu-complete
set colored-completion-prefix on
set colored-stats on
"\e[Z": menu-complete-backward
```

## Requirements
- bash-completion
- rust toolchains

## Bug report
If you encounter any issues, please report them on the GitHub issues page.

## License
This project is licensed under the GPLv3 License. See the [LICENSE](./LICENSE) file for details.