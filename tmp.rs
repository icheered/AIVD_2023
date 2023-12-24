use color_eyre::owo_colors::OwoColorize;

struct Node {
    character: char,
    is_word: bool,
    children: Vec<Box<Node>>,
}

fn construct_tree(input: Vec<&str>) -> Node {
    // Omitted for brevity
    // Construct a tree from the wordlist for faster indexing
}

fn get_number_of_characters_per_number_count(input: &str) -> [u8; 10] {
    // Omitted for brevity
}

fn is_valid(
    solution: Vec<Vec<char>>,
    input_numbers: Vec<Vec<u8>>,
    wordlist_tree: &mut Node,
    number_letter_counts: [u8; 10],
) -> bool {
    // Omitted for brevity
}

pub fn solve(input: &str, wordlist: &str) -> i64 {
    // Remove all the words from the wordlist with numbers and non-letter characters (dashes, apostrophes, etc.)
    let wordlist = wordlist
        .lines()
        .filter(|word| word.chars().all(|c| c.is_alphabetic()))
        .collect::<Vec<&str>>();

    // Create a tree of the wordlist
    let mut wordlist_tree = construct_tree(wordlist);

    // Get number of different letters each number could correspond to
    let number_letter_counts = get_number_of_characters_per_number_count(input);
    println!("Number letter counts: {:?}", number_letter_counts);

    // Input looks like this: 775237 8417415532 4831582
    // Seperate the input into a vector of vectors of numbers
    let input_numbers: Vec<Vec<u8>> = input
        .split_whitespace()
        .map(|word| {
            word.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    // Use backtracking to find the solution
    let mut solution: Vec<Vec<char>> = Vec::new();
    for word in input_numbers.iter() {
        let mut word_vec: Vec<char> = Vec::new();
        for _ in word {
            word_vec.push(' ');
        }
        solution.push(word_vec);
    }

    // Implement backtracking algorithm that loops through all characters for all words

    0
}
