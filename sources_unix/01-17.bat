@echo off
cls
rustc %* --color always 2>&1 | more
