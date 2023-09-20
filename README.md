# Overview
VersaWindowManager - an academic project to learn Rust and try some things.  

This tool allows you to save (as a json file) and restore window states.  
Restoration occurs by searching among existing visible windows for windows equal to one of saved windows with the same title, class and process name.  
In future regex will be used to match existing and saved window titles instead full text comparison.  

For example: I use this utility to maintain the perfect size and position (for me) of my browser windows, so that I can restore them if their size or position is accidentally changed.  

# Usage
Run tool with ```--save [file]``` flag to dump all visible windows data to specified file (or dump.json by default), then edit it as you like (remove unimportant windows and fix position and size of interesting windows for you) and finaly restore window data with ```--restore [file]``` flag.  

# Commands
--help - Print this help.  
--restore - Restore the state of windows (loaded from a file) for existing windows at the moment. The file path is passed as the first argument to the command, or otherwise the default path is used.  
--save - Save the data of all visible windows as a json file passed as the first command argument, or otherwise the default path is used.  
--list - List data of all visible windows.  
