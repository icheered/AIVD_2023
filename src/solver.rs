use color_eyre::owo_colors::OwoColorize;

struct Node {
    character: char,
    is_word: bool,
    children: Vec<Box<Node>>,
}

fn construct_tree(input: Vec<&str>) -> Node {
    let mut root = Node {
        character: ' ',
        is_word: false,
        children: Vec::new(),
    };

    for line in input {
        let mut current_node = &mut root;
        for character in line.chars() {
            let child_index = current_node
                .children
                .iter()
                .position(|n| n.character == character);

            match child_index {
                Some(index) => current_node = current_node.children[index].as_mut(),
                None => {
                    let new_node = Node {
                        character,
                        is_word: false,
                        children: Vec::new(),
                    };
                    current_node.children.push(Box::new(new_node));
                    current_node = current_node.children.last_mut().unwrap();
                }
            }
        }
        current_node.is_word = true;
    }

    root
}

fn get_number_of_characters_per_number_count(input: &str) -> [u8; 10] {
    // Count how many times each number appears in the input
    let mut number_counts: [u8; 10] = [0; 10];
    for character in input.chars() {
        // If space, skip
        if character == ' ' {
            continue;
        }
        let number = character.to_digit(10).unwrap() as usize;
        number_counts[number] += 1;
    }

    // Get the number of letters for each number (if the number 3 appears 9 times, then there the number 3 can correspond to 3 different letters)
    let mut number_letter_counts: [u8; 10] = [0; 10];
    for (i, number_count) in number_counts.iter().enumerate() {
        if i == 0 {
            continue;
        }
        number_letter_counts[i] = *number_count / (i as u8);
    }

    number_letter_counts
}

fn is_valid(
    solution: Vec<Vec<char>>,
    input_numbers: Vec<Vec<u8>>,
    wordlist_tree: &mut Node,
    number_letter_counts: [u8; 10],
) -> bool {
    // Check if the last word up until current character is valid (i.e. the characters are nodes in the wordlist_tree)
    let mut last_word = String::new();
    for word in solution.iter() {
        // Check if the current word is the last word
        if word == solution.last().unwrap() {
            // If the current word is the last word, then only check up until the current character
            for (i, character) in word.iter().enumerate() {
                if i == solution.last().unwrap().len() - 1 {
                    break;
                }
                last_word.push(*character);
            }
        } else {
            // If the current word is not the last word, then check the entire word
            for character in word.iter() {
                last_word.push(*character);
            }
        }
    }

    println!("Last word: {}", last_word.bright_green());

    // Traverse the tree and check if a valid node exists for the last character
    let mut current_node = wordlist_tree;
    for character in last_word.chars() {
        if character == ' ' {
            break;
        }
        let child_index = current_node
            .children
            .iter()
            .position(|n| n.character == character);

        match child_index {
            Some(index) => current_node = current_node.children[index].as_mut(),
            None => {
                println!(
                    "Invalid solution: {} is not a valid word",
                    last_word.bright_red()
                );
                return false;
            }
        }
    }

    // Compare the input numbers and characters in the solution to find how many letters each number corresponds to
    // If there is a larger number of unique letters than the number of letters the number corresponds to, then the solution is invalid
    // Use the number_letter_counts and the input_numbers and the current solution to find the number of unique letters for each number

    // Create an empty vector of vectors of chars
    let mut possible_letters: Vec<Vec<char>> = vec![Vec::new(); 10];
    // Iterate over each number in the input
    for (input_word, solution_word) in input_numbers.iter().zip(solution.iter()) {
        // Iterate over each number in the input word
        for (input_number, solution_character) in input_word.iter().zip(solution_word.iter()) {
            // If the solution character is a space, skip
            if *solution_character == ' ' {
                continue;
            }
            // If the solution character is not a space, add it to the possible letters for the current number
            // ONLY if it is not already in the possible letters for the current number
            if !possible_letters[*input_number as usize].contains(solution_character) {
                possible_letters[*input_number as usize].push(*solution_character);
            }
        }
    }

    // Check if the number of unique letters for each number is less than or equal to the number of letters the number corresponds to
    for (i, possible_letter) in possible_letters.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if possible_letter.len() > number_letter_counts[i] as usize {
            println!("Invalid solution: Number of unique characters for number {} is greater than the number of letters the number corresponds to", i);
            // Print the number of unique letters for each number
            for (i, possible_letter) in possible_letters.iter().enumerate() {
                println!("Number {}: {:?}", i, possible_letter);
            }
            return false;
        }
    }

    true
}

fn backtrack_recursive(
    solution: &mut Vec<Vec<char>>,
    input_numbers: Vec<Vec<u8>>,
    wordlist_tree: &mut Node,
    number_letter_counts: [u8; 10],
    current_word: usize,
    current_character: usize,
) -> bool {
    // If previous word last character is a space, then the solution is invalid
    if current_word > 0 && solution[current_word - 1][solution[current_word - 1].len() - 1] == ' ' {
        return false;
    }

    // If the current word is the last word, then the solution is valid
    if current_word == solution.len() - 1 && current_character == solution[current_word].len() - 1 {
        return true;
    }

    // If the current character is the last character in the current word, then move on to the next word
    if current_character == solution[current_word].len() - 1 {
        return backtrack_recursive(
            solution,
            input_numbers,
            wordlist_tree,
            number_letter_counts,
            current_word + 1,
            0,
        );
    }

    // If the current character is not the last character in the current word, then try all possible letters for the current character
    let possible_letters = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
    for letter in possible_letters.iter() {
        solution[current_word][current_character] = *letter;
        if is_valid(
            solution.clone(),
            input_numbers.clone(),
            wordlist_tree,
            number_letter_counts,
        ) {
            if backtrack_recursive(
                solution,
                input_numbers.clone(),
                wordlist_tree,
                number_letter_counts,
                current_word,
                current_character + 1,
            ) {
                return true;
            }
        }
    }

    false
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

    // solution[0][0] = 'a';
    // solution[0][1] = 'a';
    // solution[0][2] = 'z';
    // solution[1][0] = 'z';
    // println!("Solution: {:?}", solution);

    // if is_valid(
    //     solution.clone(),
    //     input_numbers.clone(),
    //     &mut wordlist_tree,
    //     number_letter_counts,
    // ) {
    //     println!("Valid solution");
    // }

    backtrack_recursive(
        &mut solution,
        input_numbers,
        &mut wordlist_tree,
        number_letter_counts,
        0,
        0,
    );

    println!("Solution: {:?}", solution);

    0
}
