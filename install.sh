#!/bin/bash

if ! command -v rustc &> /dev/null
then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

cargo install --path .

echo "Done!"
echo "warning: you may need to add ~/.cargo/bin to your PATH variable."
echo "restart your terminal and run 'easygit."