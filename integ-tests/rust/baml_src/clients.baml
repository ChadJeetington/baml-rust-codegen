client<llm> GPT35 {
  provider openai
  options {
    api_key env.OPENAI_API_KEY
    model "gpt-3.5-turbo"
  }
}

client<llm> GPT4 {
  provider openai
  options {
    api_key env.OPENAI_API_KEY
    model "gpt-4"
  }
}

client<llm> GPT4o {
  provider openai
  options {
    api_key env.OPENAI_API_KEY
    model "gpt-4-vision-preview"
  }
}

client<llm> Claude {
  provider anthropic
  options {
    api_key env.ANTHROPIC_API_KEY
    model "claude-2"
  }
}

client<llm> Gemini {
  provider google-ai
  options {
    api_key env.GOOGLE_API_KEY
    model "gemini-pro"
  }
}

client<llm> Vertex {
  provider vertex-ai
  options {
    api_key env.VERTEX_API_KEY
    model "text-bison"
    location "us-central1"
  }
}

client<llm> GPT4Audio {
  provider openai
  options {
    api_key env.OPENAI_API_KEY
    model "whisper-1"
  }
}