use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::LinkedList;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq, Debug)]
struct WordValue {
    word: String,
    value: i32,
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

fn main() {

    let mut potential_list: LinkedList<String> = LinkedList::new();
    let mut total_list: LinkedList<String> = LinkedList::new();
    let mut char_value = vec![vec![0i32; 26]; 6];
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
        total += char_value[0][ch];
        total += char_value[5][ch];
        let ch = char_vec[1] as usize - 97;
        total += char_value[1][ch];
        if char_vec[0] != char_vec[1] {
            total += char_value[5][ch];
        }
        let ch = char_vec[2] as usize - 97;
        total += char_value[2][ch];
        if char_vec[0] != char_vec[2] && char_vec[1] != char_vec[2] {
            total += char_value[5][ch];
        }
        let ch = char_vec[3] as usize - 97;
        total += char_value[3][ch];
        if char_vec[0] != char_vec[3] && char_vec[1] != char_vec[3] && char_vec[2] != char_vec[3] {
            total += char_value[5][ch];
        }
        let ch = char_vec[4] as usize - 97;
        total += char_value[4][ch];
        if char_vec[0] != char_vec[4] && char_vec[1] != char_vec[4] && char_vec[2] != char_vec[4] && char_vec[3] != char_vec[4]{
            total += char_value[5][ch];
        }
        heap.push(WordValue { word: element.clone(), value : total});
    }

    println!("{:?}", char_value);
    for _i in 1..10 {
        println!("{:?}", heap.pop());
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}