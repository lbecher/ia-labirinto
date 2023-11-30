#!/bin/bash
sudo apt update
sudo apt install -y \
    build-essential python3 curl libwayland-dev libxkbcommon-dev \
    g++ pkg-config libx11-dev libasound2-dev libudev-dev
    
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh