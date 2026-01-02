#!/usr/bin/env python3
import requests
import json

def test_llm():
    url = "http://localhost:11434/api/chat"
    payload = {
        "model": "llama2:7b",
        "messages": [
            {"role": "system", "content": "You are a fuzzing mutation engine. Mutate this input to cause edge cases. Return ONLY the raw string."},
            {"role": "user", "content": "http://example.com"}
        ]
    }
    
    try:
        response = requests.post(url, json=payload, timeout=10)
        if response.status_code == 200:
            result = response.json()
            print("✅ LLM está funcionando!")
            print("Input: http://example.com")
            print("Mutated output:", result.get("message", {}).get("content", "No content"))
        else:
            print("❌ Error HTTP:", response.status_code)
    except Exception as e:
        print("❌ Error conectando al LLM:", str(e))

if __name__ == "__main__":
    test_llm()
