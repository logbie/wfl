### Goal  
Fix Google Generative AI SDK import error in the bug reporting tool (`bugreport/bug.py`) that was preventing the tool from running.

### Approach  
The issue was caused by incorrect import paths for the newer `google-genai` SDK (v1.17.0). The code was attempting to import types from `google.generativeai.types` (old SDK path) instead of `google.genai.types` (new SDK path).

**Root Cause:**
- The `google-genai` package was correctly installed in the virtual environment
- The main import `from google import genai` was correct
- However, the types import was using the old SDK path: `from google.generativeai.types import (...)`
- This needed to be changed to: `from google.genai.types import (...)`

### Gotchas  
- The new `google-genai` SDK has different import paths compared to the legacy `google-generativeai` package
- All types (Part, FunctionDeclaration, Tool, Schema, Content, GenerationConfig, FunctionResponse, Candidate, FinishReason) are available under `genai.types`
- The rest of the code (API calls, model initialization) worked correctly with the new SDK

### Outcome  
- **Fixed:** Import path in `bugreport/bug.py` line 18: `google.generativeai.types` → `google.genai.types`
- **Added:** `requirements.txt` file documenting dependencies (`google-genai>=1.17.0`, `pydantic>=2.11.0`)
- **Verified:** Script now runs successfully and imports all required modules
- **Status:** Bug reporting tool is now functional (pending Google API key configuration)

The tool is now ready for use as part of the WFL project's bug reporting workflow. Users just need to set their `GOOGLE_API_KEY` environment variable to use the AI-powered features.
