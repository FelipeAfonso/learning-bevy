#!/bin/bash
# linux build
cargo build --release 
# windows build
cargo build --target=x86_64-pc-windows-gnu --release
#move releases to release folder
cp ./target/release/stupid-spooder-game ./release/stupid-spooder-game
cp ./target/x86_64-pc-windows-gnu/release/stupid-spooder-game.exe ./release/stupid-spooder-game.exe
# zip releases
cd ./release
zip stupid-spooder-game.zip stupid-spooder-game stupid-spooder-game.exe
cd ../
mv ./release/stupid-spooder-game.zip ./
