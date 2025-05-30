@echo off
echo.
echo ============================================================
echo   WFL Bug Reporting Tool Launcher
echo ============================================================
echo.

REM Check if we're in the right directory
if not exist "gemini_env" (
    echo ERROR: gemini_env directory not found!
    echo Please run this script from the bugreport directory.
    echo.
    pause
    exit /b 1
)

REM Check if Python environment exists
if not exist "gemini_env\Scripts\activate.bat" (
    echo ERROR: Virtual environment not properly set up!
    echo Please ensure gemini_env is properly created.
    echo.
    pause
    exit /b 1
)

echo Activating Python virtual environment...
call gemini_env\Scripts\activate.bat

echo.
echo Environment activated successfully!
echo.
echo NOTE: To use the AI features, you need a Google API key.
echo       Set it in config.py or as an environment variable.
echo.
echo Starting Bug Reporting Tool...
echo.
python bug.py

echo.
echo Bug Reporting Tool has exited.
echo.
pause
