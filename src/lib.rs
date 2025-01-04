use std::collections::hash_map::HashMap;
pub fn morse_code_hashmap() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("A", ".-"),
        ("B", "-..."),
        ("C", "-.-."),
        ("D", "-.."),
        ("E", "."),
        ("F", "..-."),
        ("G", "--."),
        ("H", "...."),
        ("I", ".."),
        ("J", ".---"),
        ("K", "-.-"),
        ("L", ".-.."),
        ("M", "--"),
        ("N", "-."),
        ("O", "---"),
        ("P", ".--."),
        ("Q", "--.-"),
        ("R", ".-."),
        ("S", "..."),
        ("T", "-"),
        ("U", "..-"),
        ("V", "...-"),
        ("W", ".--"),
        ("X", "-..-"),
        ("Y", "-.--"),
        ("Z", "--.."),
        ("0", "-----"),
        ("1", ".----"),
        ("2", "..---"),
        ("3", "...--"),
        ("4", "....-"),
        ("5", "....."),
        ("6", "-...."),
        ("7", "--..."),
        ("8", "---.."),
        ("9", "----."),
        (".", ".-.-.-"),
        (",", "--..--"),
        ("?", "..--.."),
        ("'", ".----."),
        ("!", "-.-.--"),
        ("/", "-..-."),
        ("(", "-.--."),
        (")", "-.--.-"),
        ("&", ".-..."),
        (":", "---..."),
        (";", "-.-.-."),
        ("+", ".-.-."),
        ("-", "-...-"),
        ("_", "..--.-"),
        ("=", "-...-"),
        ("\"", ".-..-."),
        ("$", "...-..-"),
        ("@", ".--.-."),
    ])
}

pub fn morse_to_english(text: &str) -> String {
    let mut result = String::new();
    let words = text.split(" ");
    let morse_code = morse_code_hashmap();
    for word in words {
        for (key, value) in morse_code.iter() {
            if &word == value {
                result.push_str((key.to_string() + " ").as_str());
                continue;
            }
        }
        if !morse_code.iter().any(|(_, v)| v == &word) {
            result.push_str(&(word.to_owned() + " "));
        }
    }
    result.to_uppercase()
}

pub fn english_to_morse(text: &str) -> String {
    let text = text.to_uppercase();
    let mut result = String::new();
    let morse_code = morse_code_hashmap();
    for c in text.chars() {
        if c == ' ' {
            continue;
        }
        match morse_code.get(c.to_string().as_str()) {
            Some(value) => result.push_str((value.to_string() + " ").as_str()),
            None => result.push_str((c.to_string() + " ").as_str()),
        }
    }
    result
}
