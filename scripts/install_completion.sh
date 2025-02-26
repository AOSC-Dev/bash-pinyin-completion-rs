#!/usr/bin/env bash
set -e

cargo build --release

# Check bash-completion
if [ ! -d /etc/bash_completion.d ]; then
    echo "Directory /etc/bash_completion.d does not exist. Please install bash-completion first."
    exit 1
fi

# Install debug version if --debug is specified
if [ "$1" = "--debug" ]; then
    echo "Installing debug version..."
    sudo cp scripts/bash_pinyin_completion_debug /etc/bash_completion.d/bash_pinyin_completion
    exit 0
fi

# Install or upgrade the binary
if [ -f /usr/bin/bash-pinyin-completion-rs ]; then
    echo "The binary /usr/bin/bash-pinyin-completion-rs already exists. Upgrading..."
else
    echo "Installing binary..."
fi
sudo cp target/release/bash-pinyin-completion-rs /usr/bin/

# Install or upgrade the script
if [ -f /etc/bash_completion.d/bash_pinyin_completion ]; then
    echo "The bash-completion script /etc/bash_completion.d/bash_pinyin_completion already exists. Upgrading..."
else
    echo "Installing bash-completion script..."
fi
sudo cp scripts/bash_pinyin_completion /etc/bash_completion.d/
