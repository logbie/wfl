try:
    import google.generativeai as genai
    print("Successfully imported google.generativeai")
except ImportError as e:
    print(f"Import error: {e}")