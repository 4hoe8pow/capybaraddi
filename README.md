# Q


## setup

```sh
#!/bin/bash

# rustupをインストール
echo "Installing rustup..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# rustupの環境を有効化
source "$HOME/.cargo/env"

# Nightlyチャンネルをインストール
echo "Installing Rust nightly..."
rustup install nightly

# Nightlyをデフォルトに設定
echo "Setting Rust nightly as default..."
rustup default nightly

# インストールされたバージョンの確認
echo "Rust installation completed. Installed version:"
rustc --version
```
