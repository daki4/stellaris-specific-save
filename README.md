# Stellaris autosave backup script

## What is this?

This program is meant to be a part of a wider suite of infrastructure tools for hosting and analyzing games, but as of now, I have completed only the backup part of it. Soon, tihs script will be integrated with hopefully a remote server to host the backed-up games, and automatically upload them to the discord server.

## Why is this a .exe?

This program was written in Rust. It can not be ran without being compiled. I chose this language, since its type system is really strict, and it can not cause memory leaks unless there is a programmer error and it will also be very performant.

With this come other benefits too. Just make a *`.env`* file, put the target dir there, and all will be good. No real additional setup would be needed.

## Setup
Create a *`.env`* file that contains the following
  - TARGET_DIR="/path/to/save/game/files" - for linux
  - TARGET_DIR="C:\\path\\to\\save\\game\\files" - for windows
