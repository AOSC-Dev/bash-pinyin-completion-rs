#!/usr/bin/env bash
set -e

cargo build --release
echo "Install binary..."
sudo cp target/release/bash-pinyin-completion-rs /usr/bin/
echo "Install bash-completion script..."
sudo cp scripts/bash_pinyin_completion /etc/bash_completion.d/
