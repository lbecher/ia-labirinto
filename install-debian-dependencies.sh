#!/bin/bash
sudo apt update
sudo apt install -y \
    build-essential python3 python3-dev g++ pkg-config curl \
    libx11-dev libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev
    
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
