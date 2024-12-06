# Ollama Translator

A library for translating text to different natural languages using Ollama.

## Description

This library enables translation of text files from one language to another using Ollama AI models capabilities. It's a tool that can be helpful for quick text translation between different languages.

## Prerequisites

- Rust installed on your system
- Ollama installed and running locally
- A compatible model (default: mixtral:8x7b)

## Installation

```bash
cargo add ollama_translator
Usage
The library can be used via command line through the provided example:


cargo run --example basic <source_text> <target_language>
Example

cargo r --example basic -- "mi estas via asistoanta permesanta vin traduki en multaj lingvoj." english
Which produces the following output:


Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
 Running `target\debug\examples\basic.exe "mi estas via asistoanta permesanta vin traduki en multaj lingvoj." english`
Translating text 'mi estas via asistoanta permesanta vin traduki en multaj lingvoj.' to 'english'
I am your assistant helping you translate into many languages.
Configuration
The library can be configured through environment variables:

OLLAMA_MODEL: The model to use for translation (default: "mixtral:8x7b")
OLLAMA_URL: Ollama endpoint (default "http://localhost:11434/api/generate")
Limitations
Translation quality depends on the model used
Some language-specific concepts may not have direct equivalents in the target language
The tool may require manual adjustments of generated text
Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request.
