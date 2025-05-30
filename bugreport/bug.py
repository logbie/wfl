import os
import json
import sys
from typing import List, Optional, Dict, Any

# Try to import config, fallback to defaults if not available
try:
    from config import GOOGLE_API_KEY, MODEL_NAME, DEFAULT_TEMPERATURE, DEFAULT_MAX_OUTPUT_TOKENS, FINAL_REPORT_MAX_OUTPUT_TOKENS, SYSTEM_INSTRUCTION
    print("✅ Loaded configuration from config.py")
except ImportError:
    print("⚠️  config.py not found, using defaults")
    GOOGLE_API_KEY = "YOUR_GOOGLE_API_KEY_HERE"
    MODEL_NAME = "gemini-1.5-pro-latest"
    DEFAULT_TEMPERATURE = 0.7
    DEFAULT_MAX_OUTPUT_TOKENS = 2048
    FINAL_REPORT_MAX_OUTPUT_TOKENS = 4096
    SYSTEM_INSTRUCTION = "You are an expert AI assistant helping users write detailed bug reports."

print("--- DIAGNOSTIC START (with google-genai) ---")
print("--- DIAGNOSTIC END ---")

try:
    from google import genai
    print("Successfully imported 'from google import genai'.")
except ImportError as e:
    print(f"Import error for new SDK 'google-genai': {e}")
    print("Please ensure 'google-genai' is installed: pip install google-genai")
    exit()

try:
    from pydantic import BaseModel, Field
    print("Successfully imported Pydantic")
except ImportError:
    print("Please install Pydantic: pip install pydantic")
    exit()

# --- Pydantic Schemas ---
class EnvironmentDetails(BaseModel):
    operating_system: Optional[str] = Field(None, description="Operating system details")
    browser: Optional[str] = Field(None, description="Browser name and version")
    software_version: Optional[str] = Field(None, description="Software version being tested")
    device: Optional[str] = Field(None, description="Device information")
    other_details: Optional[str] = Field(None, description="Additional environment details")

class BugReport(BaseModel):
    title: str = Field(..., description="Brief, descriptive title of the bug")
    description_summary: str = Field(..., description="Detailed description of the bug")
    steps_to_reproduce: List[str] = Field(..., description="Step-by-step instructions to reproduce the bug")
    expected_result: str = Field(..., description="What should happen")
    actual_result: str = Field(..., description="What actually happens")
    environment: EnvironmentDetails = Field(..., description="Environment where bug occurs")
    severity: Optional[str] = Field(None, description="Bug severity level")
    priority: Optional[str] = Field(None, description="Bug priority level")
    reporter: Optional[str] = Field(None, description="Name of person reporting the bug")
    attachments: Optional[List[str]] = Field(None, description="List of attached files")
    rate_of_reproduction: Optional[str] = Field(None, description="How often the bug occurs")

# --- Gemini API Setup ---
def initialize_gemini_client():
    print(f"🔧 Using model: {MODEL_NAME}")
    print(f"🔧 Using temperature: {DEFAULT_TEMPERATURE}")
    
    # Get API key
    api_key_to_use = os.environ.get("GOOGLE_API_KEY")
    if not api_key_to_use:
        # Try config file
        if GOOGLE_API_KEY != "YOUR_GOOGLE_API_KEY_HERE":
            api_key_to_use = GOOGLE_API_KEY
            print("🔑 Using API key from config.py")
        else:
            print("🚨 CRITICAL: No Google API key found!")
            print("   Set GOOGLE_API_KEY environment variable OR")
            print("   Edit config.py and replace YOUR_GOOGLE_API_KEY_HERE with your actual key")
            return None
    else:
        print("🔑 Using API key from environment variable")
    
    try:
        # Initialize client with API key
        client = genai.Client(api_key=api_key_to_use)
        print("✅ Gemini API client configured successfully")
        return client
    except Exception as e:
        print(f"❌ Error configuring Gemini API client: {e}")
        return None

# --- AI-Powered Bug Reporting Assistant ---
def run_bug_reporting_assistant(client):
    print("\n" + "="*60)
    print("🐛 WFL INTELLIGENT BUG REPORTING ASSISTANT")
    print("   Powered by Google Gemini AI")
    print("="*60)
    print("\nCommands:")
    print("  /quit          - Exit the assistant")
    print("  /generate_report - Generate final JSON bug report")
    print("\nTip: The AI will help guide you through comprehensive bug reporting!")
    
    if not client:
        print("\n⚠️  AI unavailable - falling back to manual mode")
        return run_manual_mode()
    
    # Initialize conversation history
    conversation_history = []
    
    # AI system prompt
    system_prompt = """You are an expert bug reporting assistant. Your job is to help users create comprehensive, professional bug reports by asking intelligent follow-up questions.

Start by acknowledging their initial bug description, then ask ONE specific question at a time to gather missing details. Focus on:
1. Steps to reproduce (be very specific)
2. Expected vs actual behavior
3. Environment details (OS, browser, software version)
4. Frequency and impact
5. Any error messages or symptoms

Keep responses conversational but professional. Ask clarifying questions that help the user think through the bug systematically. When you have enough information, suggest they type '/generate_report'.

Remember: Ask ONE focused question per response to avoid overwhelming the user."""
    
    conversation_history.append({"role": "system", "content": system_prompt})
    
    # Get initial bug description
    initial_description = input("\n➡️ Describe the bug you want to report: ")
    if initial_description.lower() == '/quit':
        return
    
    conversation_history.append({"role": "user", "content": initial_description})
    
    print("\n🤖 AI: Let me help you create a comprehensive bug report...")
    
    # Main conversation loop
    while True:
        try:
            # Prepare messages for API call (exclude system message for API call)
            messages = []
            for msg in conversation_history:
                if msg["role"] != "system":  # Skip system message for API
                    messages.append({
                        "role": msg["role"],
                        "parts": [{"text": msg["content"]}]
                    })
            
            # Add system instruction as first user message
            api_messages = [
                {
                    "role": "user", 
                    "parts": [{"text": system_prompt + "\n\nUser's bug description: " + initial_description}]
                }
            ] + messages[1:]  # Skip the original description since it's in system message now
            
            response = client.models.generate_content(
                model=MODEL_NAME,
                contents=api_messages,
                config={
                    "temperature": DEFAULT_TEMPERATURE,
                    "max_output_tokens": DEFAULT_MAX_OUTPUT_TOKENS
                }
            )
            
            if hasattr(response, 'text') and response.text:
                ai_response = response.text.strip()
                print(f"\n🤖 AI: {ai_response}")
                conversation_history.append({"role": "assistant", "content": ai_response})
                
                # Get user response
                user_input = input("\n➡️ Your response: ")
                
                if user_input.lower() == '/quit':
                    return
                
                if user_input.lower() == '/generate_report':
                    generate_final_report(client, conversation_history)
                    return
                
                conversation_history.append({"role": "user", "content": user_input})
                
            else:
                print("\n🤖 AI: I'm having trouble processing that. Could you provide more details?")
                user_input = input("\n➡️ Your response: ")
                
                if user_input.lower() == '/quit':
                    return
                
                if user_input.lower() == '/generate_report':
                    generate_final_report(client, conversation_history)
                    return
                
                conversation_history.append({"role": "user", "content": user_input})
            
        except Exception as e:
            print(f"\n🚨 AI Error: {e}")
            print("Continuing conversation...")
            user_input = input("\n➡️ Please continue describing the bug: ")
            
            if user_input.lower() == '/quit':
                return
            
            if user_input.lower() == '/generate_report':
                generate_final_report(client, conversation_history)
                return
            
            conversation_history.append({"role": "user", "content": user_input})

# --- Manual Mode Fallback ---
def run_manual_mode():
    print("\n📝 Manual Bug Reporting Mode")
    print("I'll ask you questions to gather bug information...")
    
    data = {}
    questions = [
        ("description", "🐛 Describe the bug: "),
        ("title", "📝 Brief title for this bug: "),
        ("steps", "🔄 Steps to reproduce (separate with semicolons): "),
        ("expected", "✅ What should happen: "),
        ("actual", "❌ What actually happens: "),
        ("os", "💻 Operating system: "),
        ("browser", "🌐 Browser/software version: "),
        ("severity", "⚠️ Severity (High/Medium/Low): "),
    ]
    
    for key, question in questions:
        response = input(question)
        if response.lower() == '/quit':
            return
        data[key] = response
    
    # Create manual report
    manual_report = {
        "title": data.get('title', 'Manual Bug Report'),
        "description_summary": data.get('description', ''),
        "steps_to_reproduce": data.get('steps', '').split(';') if data.get('steps') else [],
        "expected_result": data.get('expected', ''),
        "actual_result": data.get('actual', ''),
        "environment": {
            "operating_system": data.get('os', ''),
            "browser": data.get('browser', ''),
            "software_version": "Unknown",
            "device": "Unknown",
            "other_details": ""
        },
        "severity": data.get('severity', 'Medium'),
        "priority": data.get('severity', 'Medium'),
        "reporter": "Anonymous",
        "rate_of_reproduction": "Unknown"
    }
    
    filename = "bug_report_manual.json"
    with open(filename, 'w', encoding='utf-8') as f:
        json.dump(manual_report, f, indent=2)
    print(f"\n💾 Manual bug report saved to: {filename}")

# --- Generate Final Report ---
def generate_final_report(client, conversation_history):
    print("\n📝 Generating comprehensive bug report using AI...")
    
    # Extract conversation text
    conversation_text = ""
    for msg in conversation_history:
        if msg["role"] != "system":
            conversation_text += f"{msg['role']}: {msg['content']}\n"
    
    # Create structured prompt for final report
    final_prompt = f"""Based on our conversation, create a complete JSON bug report:

CONVERSATION:
{conversation_text}

Generate a JSON object with this exact structure (fill in all fields based on our conversation):
{{
  "title": "Brief, clear bug title",
  "description_summary": "Comprehensive description of the bug",
  "steps_to_reproduce": ["Step 1", "Step 2", "Step 3", "..."],
  "expected_result": "What should happen",
  "actual_result": "What actually happens",
  "environment": {{
    "operating_system": "OS details",
    "browser": "Browser/software details",
    "software_version": "Version if mentioned",
    "device": "Device details if mentioned",
    "other_details": "Any other environment info"
  }},
  "severity": "High/Medium/Low based on impact",
  "priority": "High/Medium/Low based on urgency",
  "reporter": "Name if provided, otherwise Anonymous",
  "rate_of_reproduction": "Always/Sometimes/Rarely based on conversation"
}}

Return ONLY the JSON object, no other text or explanation."""
    
    try:
        response = client.models.generate_content(
            model=MODEL_NAME,
            contents=[{"role": "user", "parts": [{"text": final_prompt}]}],
            config={
                "temperature": 0.1,
                "max_output_tokens": FINAL_REPORT_MAX_OUTPUT_TOKENS
            }
        )
        
        if hasattr(response, 'text'):
            raw_json_string = response.text.strip()
        else:
            raw_json_string = str(response)
        
        print("\n" + "="*60)
        print("📋 AI-GENERATED BUG REPORT")
        print("="*60)
        print(raw_json_string)

        # Clean potential markdown
        if raw_json_string.startswith("```json"):
            raw_json_string = raw_json_string[7:]
        if raw_json_string.endswith("```"):
            raw_json_string = raw_json_string[:-3]
        
        try:
            # Fix common escape sequence issues before parsing
            fixed_json_string = raw_json_string.strip()
            # Replace unescaped backslashes with double backslashes
            fixed_json_string = fixed_json_string.replace('\\', '\\\\')
            # But fix double-escaped backslashes (we might have created some)
            fixed_json_string = fixed_json_string.replace('\\\\\\\\', '\\\\')
            
            try:
                parsed_data = json.loads(fixed_json_string)
                bug_report = BugReport(**parsed_data)
                
                print("\n" + "="*60)
                print("✅ VALIDATED BUG REPORT")
                print("="*60)
                print(bug_report.model_dump_json(indent=2))
                
                # Save to file
                safe_title = parsed_data.get('title', 'untitled').replace(' ', '_').replace('/', '_').lower()
                safe_title = ''.join(c for c in safe_title if c.isalnum() or c == '_')
                filename = f"bug_report_{safe_title}.json"
                
                with open(filename, 'w', encoding='utf-8') as f:
                    f.write(bug_report.model_dump_json(indent=2))
                print(f"\n💾 Bug report saved to: {filename}")
                print("✅ AI-powered bug report generation complete!")
            
            except json.JSONDecodeError as e:
                print(f"⚠️  JSON parsing error after fixing escape sequences: {e}")
                print("Trying alternative parsing approach...")
                
                # Alternative approach: use a more lenient JSON parser
                import re
                
                # Extract JSON content between curly braces
                match = re.search(r'(\{.*\})', raw_json_string, re.DOTALL)
                if match:
                    json_content = match.group(1)
                    # Replace problematic escape sequences
                    json_content = re.sub(r'\\(?!["\\/bfnrt]|u[0-9a-fA-F]{4})', r'\\\\', json_content)
                    
                    try:
                        parsed_data = json.loads(json_content)
                        bug_report = BugReport(**parsed_data)
                        
                        print("\n" + "="*60)
                        print("✅ VALIDATED BUG REPORT (alternative parsing)")
                        print("="*60)
                        print(bug_report.model_dump_json(indent=2))
                        
                        # Save to file
                        safe_title = parsed_data.get('title', 'untitled').replace(' ', '_').replace('/', '_').lower()
                        safe_title = ''.join(c for c in safe_title if c.isalnum() or c == '_')
                        filename = f"bug_report_{safe_title}.json"
                        
                        with open(filename, 'w', encoding='utf-8') as f:
                            f.write(bug_report.model_dump_json(indent=2))
                        print(f"\n💾 Bug report saved to: {filename}")
                        print("✅ AI-powered bug report generation complete!")
                        return
                    except Exception as e2:
                        print(f"⚠️  Alternative parsing failed: {e2}")
                
                # If all parsing attempts fail, save the raw response
                print("Saving raw AI response...")
                with open("bug_report_ai_raw.txt", 'w', encoding='utf-8') as f:
                    f.write(raw_json_string)
                print("💾 Raw AI response saved to: bug_report_ai_raw.txt")
        
        except Exception as e:
            print(f"⚠️  Unexpected error during JSON processing: {e}")
            print("Saving raw AI response...")
            with open("bug_report_ai_raw.txt", 'w', encoding='utf-8') as f:
                f.write(raw_json_string)
            print("💾 Raw AI response saved to: bug_report_ai_raw.txt")
            
        except Exception as e:
            print(f"⚠️  Validation error: {e}")
            print("Saving raw AI response...")
            with open("bug_report_ai_raw.txt", 'w', encoding='utf-8') as f:
                f.write(raw_json_string)
            print("💾 Raw AI response saved to: bug_report_ai_raw.txt")
        
    except Exception as e:
        print(f"\n🚨 Error generating AI report: {e}")
        print("The AI conversation was saved, but report generation failed.")
        
        # Save conversation log
        with open("bug_conversation_log.txt", 'w', encoding='utf-8') as f:
            f.write("Bug Reporting Conversation Log\n")
            f.write("=" * 40 + "\n\n")
            for msg in conversation_history:
                if msg["role"] != "system":
                    f.write(f"{msg['role'].upper()}: {msg['content']}\n\n")
        print("💾 Conversation saved to: bug_conversation_log.txt")

if __name__ == "__main__":
    print("🔄 Initializing WFL Bug Reporting Tool...")
    client = initialize_gemini_client()
    try:
        run_bug_reporting_assistant(client)
    except KeyboardInterrupt: 
        print("\n👋 Goodbye!")
    except Exception as e_main:
        print(f"🚨 Main loop error: {e_main}")
        import traceback
        traceback.print_exc()
