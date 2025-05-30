import google.generativeai as genai
import inspect

print("Available in google.generativeai.types:")
for name in dir(genai.types):
    if not name.startswith('_'):  # Skip private attributes
        print(f"- {name}")

print("\nAvailable in google.generativeai:")
for name in dir(genai):
    if not name.startswith('_'):  # Skip private attributes
        print(f"- {name}")