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
    println!("3: to quit");
}

fn read_file_to_string(file_path: &str) -> String {
    let mut file = match File::open(file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path, Error::description(&why)),
        Ok(file) => file,
    };
    let mut file_as_string = String::new();
    match file.read_to_string(&mut file_as_string) {
        Err(why) => panic!("read to string failed {}: {}", file_path, Error::description(&why)),
        Ok(_) => file
    };
    return file_as_string;
}

fn main_menu_state() -> ProgramState {
    display_welcome();
    let mut user_input = String::new();
    loop {
        display_main_menu_instructions();
        io::stdin().read_line(&mut user_input);
        match user_input.trim() {
            "1" => return ProgramState::ConsoleInput,
            "2" => return ProgramState::FileInput,
            "3" => return ProgramState::Terminate,
            _ => user_input.clear()
        };
    }
}

fn console_input_state(sample_text: &mut String) -> ProgramState {
    println!("Enter sample text: ");
    io::stdin().read_line(sample_text);
    let trim_chars: &[_] = &['\n', '\r'];
    *sample_text = String::from(sample_text.trim_matches(trim_chars));
    return ProgramState::GenerateText;
}

fn file_input_state(sample_text: &mut String) -> ProgramState {
    println!("Enter file path: ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input);
    *sample_text = read_file_to_string(user_input.trim());
    return ProgramState::GenerateText;
}

fn generate_text_state(sample_text: &String) -> ProgramState {
    let mut user_input = String::new();
    let mut states: HashMap<&str, Vec<&str>> = HashMap::new();
    loop {
        println!("\nEnter order of grouping (must be a number greater than 0) or enter \'restart\' to start over");
        user_input.clear();
        io::stdin().read_line(&mut user_input);
        if user_input.trim() == "\'restart\'" {
            return ProgramState::MainMenu;
        }
        let n: usize = user_input.trim().parse()
            .expect("Please type a number!");
        if n <= 0 {
            continue;
        }
        for i in n..sample_text.len() {
            if i + n > sample_text.len() {
                break;
            }
            let start_of_string = i - n;
            let end_of_string = i;
            let state: &str = &sample_text[start_of_string..end_of_string];
            let possible_next_states = states.entry(state).or_insert(Vec::new());
            possible_next_states.push(&sample_text[(i)..(i + n)]);
        }

        // now generate some text
        println!("\nGenerating text with order {}:", n);
        let starting_state_index = rand::thread_rng().gen_range(0, sample_text.len() - n);
        let mut state = &sample_text[starting_state_index..starting_state_index + n];
        for letter_index in 0..(sample_text.len() / n) {
            print!("{}", state);
            let next_possible_states = states.entry(state).or_insert(Vec::new());
            if next_possible_states.len() != 0 {
                let next_state_index = rand::thread_rng().gen_range(0, next_possible_states.len());
                state = next_possible_states[next_state_index];
            }
            else {
                state = &sample_text[starting_state_index..starting_state_index + n];
            }
        }
        println!("");
    }
    // print the table out
    /*for (state, possible_states) in &states {
        print!("\n{} ->", state);
        for next_state in possible_states {
            print!(",{}", next_state);
        }
    }*/
}

fn main() {
    let mut sample_text = String::new();
    let mut current_program_state = ProgramState::MainMenu;
    loop {
        current_program_state = match current_program_state {
            ProgramState::MainMenu     => main_menu_state(),
            ProgramState::ConsoleInput => console_input_state(&mut sample_text),
            ProgramState::FileInput    => file_input_state(&mut sample_text),
            ProgramState::GenerateText => generate_text_state(&sample_text),
            ProgramState::Terminate    => break
        };
    }
}

