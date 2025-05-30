# WFL Bug Reporting Tool

An intelligent AI-powered bug reporting assistant for the WFL project, built with Google Gemini AI.

## Features

- 🤖 **AI-Powered**: Uses Google Gemini to guide users through comprehensive bug reporting
- 📝 **Structured Output**: Generates standardized JSON bug reports
- 🛠️ **Interactive**: Asks clarifying questions to gather complete information
- 💾 **Auto-Save**: Automatically saves generated reports to JSON files
- ⚙️ **Configurable**: Easy configuration through `config.py`

## Quick Start

### Option 1: Use the Batch File (Windows)
```cmd
# Navigate to the bugreport directory
cd c:/path/to/wfl/bugreport

# Double-click or run:
start_bug_reporter.bat
```

### Option 2: Manual Setup
```cmd
# Navigate to bugreport directory
cd c:/path/to/wfl/bugreport

# Activate virtual environment
.\gemini_env\Scripts\Activate.ps1

# Run the tool
python bug.py
```

## Setup Requirements

### 1. Google API Key
You need a Google API key to use the AI features. Get one from [Google AI Studio](https://aistudio.google.com/).

**Option A: Environment Variable (Recommended)**
```cmd
set GOOGLE_API_KEY=your_actual_api_key_here
```

**Option B: Config File**
Edit `config.py` and replace:
```python
GOOGLE_API_KEY = "YOUR_GOOGLE_API_KEY_HERE"
```
with:
```python
GOOGLE_API_KEY = "your_actual_api_key_here"
```

### 2. Dependencies
All dependencies are already installed in the `gemini_env` virtual environment:
- `google-genai>=1.17.0`
- `pydantic>=2.11.0`

## Usage

1. **Start the tool** using one of the methods above
2. **Describe your bug** when prompted
3. **Answer AI questions** to provide complete details
4. **Type `/generate_report`** when you've provided enough information
5. **Review the generated JSON** bug report
6. **Find your saved report** in the current directory

### Commands
- `/quit` - Exit the tool
- `/generate_report` - Generate final JSON bug report

## Configuration

Edit `config.py` to customize:

```python
# API Configuration
GOOGLE_API_KEY = "your_key_here"

# Model Settings
MODEL_NAME = "gemini-1.5-pro-latest"
DEFAULT_TEMPERATURE = 0.7
DEFAULT_MAX_OUTPUT_TOKENS = 2048

# Custom System Instructions
SYSTEM_INSTRUCTION = "Your custom instructions..."
```

## Generated Reports

Reports are saved as JSON files with structured data:
- **Title**: Brief bug description
- **Description**: Detailed summary
- **Steps to Reproduce**: Step-by-step instructions
- **Expected/Actual Results**: What should vs. what actually happens
- **Environment**: OS, browser, software versions, etc.
- **Metadata**: Severity, priority, reproduction rate

## Troubleshooting

### Import Errors
- Ensure you're in the virtual environment
- All dependencies should be pre-installed

### API Key Issues
- Check that your API key is valid
- Ensure it's properly set in config.py or environment variable
- Verify you have quota remaining in Google AI Studio

### Virtual Environment Issues
- Re-create the environment if needed:
  ```cmd
  python -m venv gemini_env
  .\gemini_env\Scripts\Activate.ps1
  pip install -r requirements.txt
  ```

## Contributing

This tool is part of the WFL project. Follow the WFL development guidelines:
- Create diary entries for changes in `/Dev diary/`
- Update documentation as needed
- Test changes thoroughly

## File Structure

```
bugreport/
├── bug.py                    # Main application
├── config.py                 # Configuration file
├── requirements.txt          # Python dependencies
├── start_bug_reporter.bat    # Windows launcher
├── README.md                 # This file
├── gemini_env/              # Python virtual environment
└── *.json                   # Generated bug reports
```

---

**Part of the WFL Project** - Making bug reporting intelligent and comprehensive!
