use std::env;
use std::process;
use ollama_translator::translate_from_string;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: basic_string <text> <target_language>");
        process::exit(1);
    }

    let text = &args[1];
    let target_language = &args[2];

    println!("Translating code '{}' to '{}'", text, target_language);

    // Call the translation function
    match translate_from_string(text, target_language) {
        Ok(translated_text) => {
            println!("{}", translated_text);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}

