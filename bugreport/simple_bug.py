import os
import json
import google.generativeai as genai

# Configuration
GEMINI_API_KEY = os.environ.get("GEMINI_API_KEY")
if not GEMINI_API_KEY:
    GEMINI_API_KEY = "AIzaSyAY_ENJNvEj1xHqTIcQwsDnbtrSmrU_2kQ"  # Default key from original script

MODEL_NAME = "gemini-1.5-pro-latest"  # Using a more widely available model
TEMPERATURE = 0.7
MAX_OUTPUT_TOKENS = 2048

# System instruction
SYSTEM_INSTRUCTION = """
You are an expert AI assistant helping users write detailed and thoughtful bug reports for the WFL programming language.
Your goal is to analyze initial bug descriptions, ask clarifying questions if information is missing,
and then synthesize all gathered information into a structured bug report.

Focus on obtaining all necessary details like:
- Steps to reproduce
- Environment details
- Expected results
- Actual results
- Severity and impact
- Possible causes

Maintain a friendly and helpful tone. When you have enough information, summarize the bug report in a structured format.
"""

def initialize_gemini():
    """Initialize the Gemini model"""
    try:
        genai.configure(api_key=GEMINI_API_KEY)
        
        model = genai.GenerativeModel(
            model_name=MODEL_NAME,
            generation_config={
                "temperature": TEMPERATURE,
                "max_output_tokens": MAX_OUTPUT_TOKENS,
            },
            system_instruction=SYSTEM_INSTRUCTION
        )
        return model
    except Exception as e:
        print(f"Error initializing Gemini: {e}")
        return None

def run_bug_assistant():
    """Run the bug reporting assistant"""
    model = initialize_gemini()
    if not model:
        print("Failed to initialize Gemini model. Exiting.")
        return
    
    print("=== WFL Bug Reporting Assistant ===")
    print("This assistant will help you create a detailed bug report for WFL.")
    print("Type 'exit' to quit at any time.\n")
    
    # Start a chat session
    chat = model.start_chat()
    
    # Get initial bug description
    initial_description = input("Please describe the bug you encountered: ")
    if initial_description.lower() == 'exit':
        print("Exiting. Goodbye!")
        return
    
    # Send initial message to Gemini
    try:
        response = chat.send_message(initial_description)
        print(f"\nAssistant: {response.text}\n")
    except Exception as e:
        print(f"Error communicating with Gemini: {e}")
        return
    
    # Continue conversation until user exits
    while True:
        user_input = input("Your response (or 'exit' to quit): ")
        if user_input.lower() == 'exit':
            print("Exiting. Goodbye!")
            break
        
        try:
            response = chat.send_message(user_input)
            print(f"\nAssistant: {response.text}\n")
        except Exception as e:
            print(f"Error communicating with Gemini: {e}")
            break

if __name__ == "__main__":
    run_bug_assistant()