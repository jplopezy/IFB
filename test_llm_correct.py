#!/usr/bin/env python3
import requests
import json

def test_llm():
    # Usar la API correcta de Ollama
    url = "http://localhost:11434/api/generate"
    payload = {
        "model": "llama2:7b",
        "prompt": "You are a fuzzing mutation engine. Mutate this input to cause edge cases. Return ONLY the raw string.\n\nInput: http://example.com\n\nOutput:",
        "stream": False
    }
    
    try:
        response = requests.post(url, json=payload, timeout=30)
        if response.status_code == 200:
            result = response.json()
            print("✅ LLM está funcionando!")
            print("Input: http://example.com")
            print("Mutated output:", result.get("response", "No response").strip())
        else:
            print("❌ Error HTTP:", response.status_code, response.text[:200])
    except Exception as e:
        print("❌ Error conectando al LLM:", str(e))

if __name__ == "__main__":
    test_llm()
