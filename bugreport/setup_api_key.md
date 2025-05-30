# 🔑 Setting Up Your Google API Key for WFL Bug Reporter

## Step 1: Get a Free API Key from Google

1. **Go to Google AI Studio:** https://aistudio.google.com/
2. **Sign in** with your Google account
3. **Click "Get API Key"** in the top right
4. **Create new project** or select existing one
5. **Copy the API key** (starts with `AIza...`)

## Step 2: Configure the API Key

### Option A: Environment Variable (Recommended - More Secure)
```cmd
set GOOGLE_API_KEY=your_actual_api_key_here
```

### Option B: Edit config.py File
Replace the placeholder in `bugreport/config.py`:
```python
GOOGLE_API_KEY = "AIzaSy_YOUR_ACTUAL_KEY_HERE"
```

## Step 3: Test the Configuration

Run the bug reporter:
```cmd
cd c:/logbie/wfl/bugreport
.\gemini_env\Scripts\Activate.ps1
python bug.py
```

## ⚠️ Important Notes

- **Free tier:** Google gives you free API credits to start
- **Keep it secret:** Never share your API key publicly
- **Environment variable preferred:** More secure than storing in files

## 🚨 If You Get Errors

- **"API key not valid"**: The key is wrong or expired
- **"Quota exceeded"**: You've used up free credits (very generous limits)
- **"Permission denied"**: Enable the Generative AI API in Google Cloud Console

## 🎯 Quick Test

Once configured, your bug reporter should show:
```
✅ Gemini API client configured successfully
🤖 AI: Thank you for describing the bug. Let me ask some follow-up questions...
```

Instead of API errors!
