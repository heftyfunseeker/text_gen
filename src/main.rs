extern crate rand;
use std::error::Error;
use rand::Rng;
use std::collections::HashMap;
use std::vec::Vec;
use std::io;
use std::fs::File;
use std::io::prelude::*;

enum ProgramState {
    MainMenu,
    ConsoleInput,
    FileInput,
    GenerateText,
    Terminate
}

fn display_welcome() {
    println!("Generate text using Markov Chains!");
    println!("This program uses sample text to create a statistical model for producing similar looking text.");
    println!("The algorithm uses letters and will use all characters except \\n \\r.");
    println!("By increasing the order of groupings (number of letters) the text will look more like the input.");
}

fn display_main_menu_instructions() {
    println!("\nSelect input source by entering the corresponding number:");
    println!("1: Console input");
    println!("2: File input");
    println!("3: Toggle state table printing");
    println!("4: to quit");
}

fn read_file_to_string(file_path: &str) -> Result<String, io::Error> {
    let mut file = match File::open(file_path) {
        Err(why) => return Err(why),
        Ok(file) => file,
    };
    let mut file_as_string = String::new();
    match file.read_to_string(&mut file_as_string) {
        Err(why) => return Err(why),
        Ok(_) => file
    };
    return Ok(file_as_string);
}

fn main_menu_state(enable_state_table_printing: &mut bool) -> ProgramState {
    display_welcome();
    let mut user_input = String::new();
    loop {
        display_main_menu_instructions();
        io::stdin().read_line(&mut user_input);
        match user_input.trim() {
            "1" => return ProgramState::ConsoleInput,
            "2" => return ProgramState::FileInput,
            "3" => *enable_state_table_printing = !*enable_state_table_printing,
            "4" => return ProgramState::Terminate,
            _ => user_input.clear()
        };
        user_input.clear()
    }
}

fn console_input_state(sample_text: &mut String) -> ProgramState {
    println!("Enter sample text: ");
    sample_text.clear();
    io::stdin().read_line(sample_text);
    let trim_chars: &[_] = &['\n', '\r'];
    *sample_text = String::from(sample_text.trim_matches(trim_chars));
    return ProgramState::GenerateText;
}

fn file_input_state(sample_text: &mut String) -> ProgramState {
    println!("Enter file path: ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input);
    *sample_text = match read_file_to_string(user_input.trim()) {
        Ok(v) => v,
        Err(e) => return ProgramState::MainMenu
    };
    return ProgramState::GenerateText;
}

fn get_order_from_user(user_input: &String) -> usize {
    let order: usize = match user_input.trim().parse() {
        Ok(v) => v,
        Err(e) => 0
    };
    return order;
}

fn generate_state_table(sample_text: &str, order: usize) -> HashMap<&str, Vec<&str>> {
    let mut states: HashMap<&str, Vec<&str>> = HashMap::new();
    for i in order..sample_text.len() {
        let state: &str = &sample_text[(i - order)..i];

        let next_state: &str;
        if i + order < sample_text.len() {
            next_state = &sample_text[i..(i + order)];
        }
        else {
            // end of string
            next_state = &sample_text[i..(sample_text.len())];
        }

        // insert the next state first because of the mutable handle below
        if (states.contains_key(next_state) == false) {
            states.insert(next_state, Vec::new());
        }
        let possible_next_states = states.entry(state).or_insert(Vec::new());
        possible_next_states.push(&next_state);
    }
    return states;
}

fn generate_text_from_state_table(
    states: &HashMap<&str, Vec<&str>>,
    generated_string_length: usize
) -> String {
    let mut generated_string = String::new();
    let mut state: &str = states.keys().next().expect("where's the states!");
    for letter_index in 0..generated_string_length {
        generated_string.push_str(state);
        let next_possible_states = states.get(state).expect("where's the next states!");
        if next_possible_states.len() != 0 {
            let next_state_index = rand::thread_rng().gen_range(0, next_possible_states.len());
            state = next_possible_states[next_state_index];
        }
        else {
            let next_state_index = rand::thread_rng().gen_range(0, states.keys().len());
            state = states.keys().nth(next_state_index).expect("where's the nth state!");
        }
    }
    return generated_string;
}

fn generate_text_state(sample_text: &String, enable_state_table_printing: bool) -> ProgramState {
    let mut user_input = String::new();
    loop {
        println!("\nEnter order of grouping where (0 < order < sample_length) or enter \'restart\' to start over");
        user_input.clear();
        io::stdin().read_line(&mut user_input);
        if user_input.trim() == "restart" {
            return ProgramState::MainMenu;
        }

        // get a valid order for state table generation
        let n = get_order_from_user(&user_input);
        if n == 0 || n >= sample_text.len() {
            continue;
        }

        let mut states = generate_state_table(sample_text, n);

        // now generate some text
        println!("\nGenerating text with order {}:", n);
        let generated_string = generate_text_from_state_table(&states, sample_text.len() / n);
        println!("{} length: {}", generated_string, generated_string.len());

        // print state table if enabled
        if (enable_state_table_printing) {
            println!("Printing state table");
            for (state, possible_states) in &states {
                print!("\n{} ->", state);
                for next_state in possible_states {
                    print!(",{}", next_state);
                }
            }
        }
    }
}

fn main() {
    let mut enable_state_table_printing = false;
    let mut sample_text = String::new();
    let mut current_program_state = ProgramState::MainMenu;
    loop {
        current_program_state = match current_program_state {
            ProgramState::MainMenu     => main_menu_state(&mut enable_state_table_printing),
            ProgramState::ConsoleInput => console_input_state(&mut sample_text),
            ProgramState::FileInput    => file_input_state(&mut sample_text),
            ProgramState::GenerateText => generate_text_state(&sample_text, enable_state_table_printing),
            ProgramState::Terminate    => break
        };
    }
}

