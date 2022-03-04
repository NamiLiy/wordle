#![feature(drain_filter)]
use std::collections::LinkedList;


use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq, Debug)]
struct WordValue {
    word: String,
    value: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum LetterColor {
    Grey,
    Black,
    Yellow,
    Green
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for WordValue {
    fn cmp(&self, other: &WordValue) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.value.cmp(&other.value)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for WordValue {
    fn partial_cmp(&self, other: &WordValue) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//cargo +nightly run --release

fn main() {

    let mut potential_list: LinkedList<String> = LinkedList::new();
    let mut total_list: LinkedList<String> = LinkedList::new();
    let mut letter_array = vec![vec![LetterColor::Grey; 26]; 5];
    let mut selected_letter_list: LinkedList<char> = LinkedList::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./word_list.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                // println!("{}", ip);
                potential_list.push_back(ip.clone());
                total_list.push_back(ip.clone());
            }
        }
    }

    for tries in 0..6 {

        let mut char_value = vec![vec![0i32; 26]; 6];

        for element in potential_list.iter() {
            let char_vec: Vec<char> = element.clone().chars().collect();
            let ch = char_vec[0] as usize - 97;
            char_value[0][ch] += 1;
            let ch = char_vec[1] as usize - 97;
            char_value[1][ch] += 1;
            let ch = char_vec[2] as usize - 97;
            char_value[2][ch] += 1;
            let ch = char_vec[3] as usize - 97;
            char_value[3][ch] += 1;
            let ch = char_vec[4] as usize - 97;
            char_value[4][ch] += 1;
        }
    
        for i in 0..26 {
            char_value[5][i] = char_value[0][i] + char_value[1][i] + char_value[2][i] + char_value[3][i] + char_value[4][i];
        }
    
        let mut heap = BinaryHeap::new();
    
        for element in total_list.iter() {
            let mut total = 0;
            let char_vec: Vec<char> = element.clone().chars().collect();
            let ch = char_vec[0] as usize - 97;

            if letter_array[0][ch] != LetterColor::Green {
                total += char_value[0][ch];
            }
            if !selected_letter_list.iter().any(|&t| t==char_vec[0]) {
                total += char_value[5][ch];
            }


            let ch = char_vec[1] as usize - 97;

            if letter_array[1][ch] != LetterColor::Green {
                total += char_value[1][ch];
            }
            if !selected_letter_list.iter().any(|&t| t==char_vec[1]) {
                if char_vec[0] != char_vec[1] {
                    total += char_value[5][ch];
                }
            }

            
            let ch = char_vec[2] as usize - 97;

            if letter_array[2][ch] != LetterColor::Green {
                total += char_value[2][ch];
            }
            if !selected_letter_list.iter().any(|&t| t==char_vec[2]) {
                if char_vec[0] != char_vec[2] && char_vec[1] != char_vec[2] {
                    total += char_value[5][ch];
                }
            }

            let ch = char_vec[3] as usize - 97;

            if letter_array[3][ch] != LetterColor::Green {
                total += char_value[3][ch];
            }
            if !selected_letter_list.iter().any(|&t| t==char_vec[3]) {
                if char_vec[0] != char_vec[3] && char_vec[1] != char_vec[3] && char_vec[2] != char_vec[3] {
                    total += char_value[5][ch];
                }
            }

            let ch = char_vec[4] as usize - 97;

            if letter_array[4][ch] != LetterColor::Green {
                total += char_value[4][ch];
            }
            if !selected_letter_list.iter().any(|&t| t==char_vec[4]) {
                if char_vec[0] != char_vec[4] && char_vec[1] != char_vec[4] && char_vec[2] != char_vec[4] && char_vec[3] != char_vec[4]{
                    total += char_value[5][ch];
                }
            }

            heap.push(WordValue { word: element.clone(), value : total});
        }
    
        println!("{:?}", char_value);
    
        println!("Potential words remaining : {}", potential_list.len());

        let suggested_word;
        if potential_list.len() < 3 {
            suggested_word = Some(WordValue {word: potential_list.pop_front().unwrap(),value : -1 })
        } else {
            suggested_word = heap.pop();
        }

        println!("Play the word : {:?}", suggested_word);
    
        // for _i in 1..10 {
        //     println!("{:?}", heap.pop());
        // }
    
        println!("Enter result");
    
        let mut keyboard_string = String::new();
        let b1 = std::io::stdin().read_line(&mut keyboard_string).unwrap();
        // println!("User entered , {}", keyboard_string);
        if b1 != 6 {
            println!("Please enter a five word charachter: Charachters entered , {}", b1 - 1);
        }
        if keyboard_string == "ggggg\n" {
            println!("Result obtained in {} tries", tries + 1);
            return
        }
        let input_char_vec: Vec<char> = keyboard_string.clone().chars().collect();
        let suggested_char_vec: Vec<char> = suggested_word.unwrap().word.clone().chars().collect();
        for i in 0..5 {
            let ch = suggested_char_vec[i] as usize - 97;
            if input_char_vec[i] == 'b' {
                for j in 0..5 {
                    letter_array[j][ch] = LetterColor::Black
                }
            }
            if input_char_vec[i] == 'y' {
                for j in 0..5 {
                    if letter_array[j][ch] != LetterColor::Green ||  letter_array[j][ch] != LetterColor::Black {
                        letter_array[j][ch] = LetterColor::Yellow
                    }
                }
                letter_array[i][ch] = LetterColor::Black;
                if !selected_letter_list.iter().any(|&t| t==suggested_char_vec[i]) {
                    selected_letter_list.push_back(suggested_char_vec[i])
                } 
            }
            if input_char_vec[i] == 'g' {
                for j in 0..26 {
                    letter_array[i][j] = LetterColor::Black
                }
                letter_array[i][ch] = LetterColor::Green;
                if !selected_letter_list.iter().any(|&t| t==suggested_char_vec[i]) {
                    selected_letter_list.push_back(suggested_char_vec[i])
                } 
            }
        }
    
        println!("{:?}", letter_array);
        println!("{:?}", selected_letter_list);
    
        let _p = potential_list.drain_filter(|word| remove_word(word.clone(), &selected_letter_list, &letter_array)).collect::<LinkedList<_>>();

    } 

    

    
}

fn remove_word(word: String, selected_letter_list: &LinkedList<char>, letter_array: &Vec<Vec<LetterColor>>) -> bool {
    let char_vec: Vec<char> = word.clone().chars().collect();
    for element in selected_letter_list {
        if char_vec[0] != *element && char_vec[1] != *element && char_vec[2] != *element && char_vec[3] != *element && char_vec[4] != *element{
            return true
        }
    }
    for i in 0..5 {
        let ch = char_vec[i] as usize - 97;
        if letter_array[i][ch] == LetterColor::Black {
            return true
        }
    }
    
    // if char_vec[0] == 't' {
    //     return true
    // }
    false
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}