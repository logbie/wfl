"""
Configuration file for the WFL Bug Reporting Tool
"""

# Google API Key Configuration
# Option 1: Set your API key here (replace the placeholder)
GOOGLE_API_KEY = ""

# Option 2: Set as environment variable (recommended for security)
# Set GOOGLE_API_KEY environment variable in your system

# Gemini Model Configuration
MODEL_NAME = "gemini-1.5-pro-latest"
DEFAULT_TEMPERATURE = 0.7
DEFAULT_MAX_OUTPUT_TOKENS = 2048
FINAL_REPORT_MAX_OUTPUT_TOKENS = 4096

# System Instructions
SYSTEM_INSTRUCTION = (
    "You are an expert AI assistant helping users write detailed and thoughtful bug reports. "
    "Your goal is to analyze initial bug descriptions, ask clarifying questions if information is missing (using function calls when appropriate), "
    "and then synthesize all gathered information into a structured JSON bug report. "
    "Focus on obtaining all necessary details like steps to reproduce, environment, expected results, and actual results. "
    "Maintain a friendly and helpful tone. Let the user know they can type '/generate_report' when they feel they have provided enough information."
)
