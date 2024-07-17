#!/bin/bash
# linux build
cargo build --release 
# windows build
cargo build --target=x86_64-pc-windows-gnu --release
#move releases to release folder
cp ./target/release/stupid-spooder-game ./release/stupid-spooder-game
cp ./target/x86_64-pc-windows-gnu/release/stupid-spooder-game.exe ./release/stupid-spooder-game.exe
cp ./assets/**/*.{png,mp3,ttf} ./release/ -r --parents
# zip releases
cd ./release
zip stupid-spooder-game-windows.zip stupid-spooder-game.exe assets -r
zip stupid-spooder-game-linux.zip stupid-spooder-game assets -r
cd ../
mv ./release/stupid-spooder-game-*.zip ./
