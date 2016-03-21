extern crate rand;
use rand::Rng;
use std::collections::HashMap;
use std::vec::Vec;

fn main() {
    let sample_text = String::from("aa ba bbaac abc");
    let mut state_to_possible_states: HashMap<u8, Vec<u8>> = HashMap::new();

    let sample_text_as_bytes = sample_text.into_bytes();

    for byte_index in 1..sample_text_as_bytes.len() {
        let curr_byte_index = byte_index - 1;
        let byte = sample_text_as_bytes[byte_index];
        let curr_byte = sample_text_as_bytes[curr_byte_index];
        let list_of_next_states = state_to_possible_states.entry(curr_byte).or_insert(Vec::new());
        list_of_next_states.push(byte);
    }

    // print the table out
    for (letter, possible_states) in &state_to_possible_states {
        //let s = String::from_utf8(*possible_states);
        print!("\n{} ->", *letter as char);
        for next_state in possible_states {
            print!(" {}", *next_state as char);
        }
    }
    println!("");

    // now generate some text
    let starting_state_index = rand::thread_rng().gen_range(0, sample_text_as_bytes.len());
    let mut state = sample_text_as_bytes[starting_state_index];
    for letter_index in 0..16 {
        print!("{}", state as char);
        let next_possible_states = state_to_possible_states.entry(state).or_insert(Vec::new());
        let next_state_index = rand::thread_rng().gen_range(0, next_possible_states.len());
        state = next_possible_states[next_state_index];
    }

}

