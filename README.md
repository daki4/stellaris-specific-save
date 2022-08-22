# Stellaris autosave backup script

## What is this?

This program is meant to be a part of a wider suite of infrastructure tools for hosting and analyzing games, but as of now, I have completed only the backup part of it. Soon, tihs script will be integrated with hopefully a remote server to host the backed-up games, and automatically upload them to the discord server.

## Why is this a .exe?

This program was written in Rust. It can not be ran without being compiled. I chose this language, since its type system is really strict, and it can not cause memory leaks unless there is a programmer error and it will also be very performant.

With this come other benefits too. Just make a *`.env`* file, put the target dir there, and all will be good. No real additional setup would be needed.

## Setup
Unzip the `release.zip` file in the directory you like, and go edit the `settings.json` file. 
 - You need to change the `target_dir` to reflect the path you want to have your saves located at. 
 - `delay_seconds` is a setting that if set, will back up every X seconds all the autosaves it can find.
 - `years_passed` is a setting that will back up the autosaves only EVERY X amount of years. ~~For example: `years_passed` is set at 5, every 5 years, the script will back up an autosave.~~

**The storage path should be changed before operation, since it has a default value right now.**

## Features

You can set a delay as to every how many seconds the program should back files up<br>
**Default value: 0**

You can set every how many years the program should back a save up.<br>
**Default value: 5**