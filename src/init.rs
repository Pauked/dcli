
/*


 */

use std::io::Write;

use crate::constants;

pub fn init() {

    // Wait for user input
    loop {
        let input = prompt(constants::PROMPT_CONFIG);
        run_option(&input);
    }
}


fn run_option(option: &str) {
    /*
    Config
    - IWADs
    - Engine paths?
     */
}


fn prompt(prompt: &str) -> String {
    let mut line = String::new();
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}