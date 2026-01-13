# bash-pinyin-completion-rs

Completion script for pinyin and romaji, matcher based on [IbPinyinLib](https://github.com/Chaoses-Ib/IbPinyinLib)

## Installation

**Clone the project**

```bash
git clone https://github.com/AOSC-Dev/bash-pinyin-completion-rs
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

## Configuring Pinyin Schema

`bash-pinyin-completion-rs` supports multiple Pinyin schemes:

- **Quanpin**: Quanpin (full Pinyin) without tone marking - e.g., "zhongguo" for "中国"
- **ShuangpinAbc**: Shuangpin (double Pinyin, or two-letter Pinyin) - 智能 ABC / Intelligent ABC scheme
- **ShuangpinJiajia**: Shuangpin (double Pinyin, or two-letter Pinyin) - 拼音加加 / Pinyin Jiajia scheme
- **ShuangpinMicrosoft**: Shuangpin (double Pinyin, or two-letter Pinyin) - 微软拼音 / MSPY scheme
- **ShuangpinThunisoft**: Shuangpin (double Pinyin, or two-letter Pinyin) - 紫光拼音 / Thunisoft scheme
- **ShuangpinXiaohe**: Shuangpin (double Pinyin, or two-letter Pinyin) - 小鹤 / Xiaohe scheme
- **ShuangpinZrm**: Shuangpin (double Pinyin, or two-letter Pinyin) - 自然码 / Ziranma scheme

You may configure the active scheme/schema with the `PINYIN_COMP_MODE` variable,
typically set in `.bashrc`. If not set or value is invalid, `bash-pinyin-completion-rs`
defaults to `Quanpin`.

For example, to enable the 小鹤 / Xiaohe Shuangpin scheme:

```bash
export PINYIN_COMP_MODE="ShuangpinXiaohe"
```

To use Quanpin together with Shuangpin (Xiaohe):

```bash
export PINYIN_COMP_MODE="Quanpin,ShuangpinXiaohe"
```

### Notes on Completion Modes

- Prefix matching (e.g., "zg" for "中国") is enabled by default with Quanpin,
  but will be disabled if any Shuangpin schema is enabled.
- Mixing Shuangpin schemas is not supported -
  if multiple Shuangpin schemas are enabled, only the first one will take effect.

## Configuring Japanese Romaji

`bash-pinyin-completion-rs` also supports Japanese romaji (ローマ字) completion using [Hepburn romanization](https://en.wikipedia.org/wiki/Hepburn_romanization).

Two modes are available:

- **Romaji**: Fast mode (~37ms startup) - supports kana and individual kanji readings. E.g., "ohayo" for "おはよう", "jojo" for "ジョジョ"
- **RomajiFull**: Full mode (~374ms startup) - includes word dictionary for compound kanji words. E.g., "kyou" for "今日"

To enable Japanese romaji completion (fast mode, recommended):

```bash
export PINYIN_COMP_MODE="Romaji"
```

To enable full romaji mode with word dictionary:

```bash
export PINYIN_COMP_MODE="RomajiFull"
```

To use both Pinyin and Romaji together:

```bash
export PINYIN_COMP_MODE="Quanpin,Romaji"
```

### Notes on Romaji Mode

- **Romaji** (fast mode): Supports hiragana, katakana, and individual kanji with known readings. Recommended for most users.
- **RomajiFull** (full mode): Additionally supports compound word readings (e.g., 今日→kyou). Has longer startup time due to word dictionary loading.
- The Romaji dictionary adds approximately 4.8 MiB to the binary size.

## Bug report

If you encounter any issues, please report them on the GitHub issues page.

## License

This project is licensed under the GPLv3 License. See the [LICENSE](./LICENSE) file for details.
