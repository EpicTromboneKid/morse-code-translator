use std::env;
fn main() {
    let args = env::args().collect::<Vec<String>>();
    //dbg!(&args);
    let choice = &args[1];
    let text = &args[2];
    //println!("{:?}", &args[2]);

    match choice.as_str() {
        "me" | "mtoe" | "morse_to_english" => {
            //println!("morse to english: ");
            println!("{}", morse_code_translator::morse_to_english(text));
        }
        "em" | "etom" | "english_to_morse" => {
            //println!("english to morse:");
            println!(
                "{}",
                morse_code_translator::english_to_morse(text.to_uppercase().as_str())
            );
        }
        _ => println!("Invalid choice, use me for morse to english or em for english to morse"),
    }
}
