@echo off
echo Testing WFL REPL functionality...
echo.

REM Run the REPL and pipe in commands
(
echo .help
echo store 5 as x
echo display x
echo display x plus 10
echo .exit
) | cargo run

echo.
echo Test completed.
