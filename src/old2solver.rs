// Struct to hold each input 'word' (list of numbers)
struct Word {
    numbers: Vec<u8>,
    possible_words: Vec<String>,
}

fn filter_words_globally(input_numbers: &Vec<Vec<u8>>, possible_words: &mut Vec<&str>) {
    // Get list of unique input word lengths
    let mut word_lengths: Vec<usize> = input_numbers.iter().map(|word| word.len()).collect();
    word_lengths.sort_unstable();
    word_lengths.dedup();

    // Remove any words from the wordlist that have a length not in the word_lengths vector
    possible_words.retain(|word| word_lengths.contains(&word.len()));

    // Remove any words with characters not in the alphabet (a-z) (remove numbers and symbols)
    possible_words.retain(|word| {
        word.chars()
            .all(|c| c.is_ascii_alphabetic() && c.is_ascii_lowercase())
    });
}

fn refine_possible_words(input_words: &mut Vec<Word>, possible_letters: &Vec<Vec<char>>) {
    for word in input_words.iter_mut() {
        word.possible_words.retain(|possible_word| {
            let mut letter_map = vec![None; 10];
            let mut used_letters_for_1 = Vec::new();

            for (number, letter) in word.numbers.iter().zip(possible_word.chars()) {
                let number = *number as usize;

                // Check for number 7 and 8 specific rules
                if number == 7 || number == 8 {
                    // If the letter is not in the possible letters for this number, discard the word
                    if !possible_letters[number].contains(&letter) {
                        return false;
                    }

                    match letter_map[number] {
                        Some(mapped_letter) => {
                            // If the number should map to the same letter but doesn't, discard the word.
                            if mapped_letter != letter {
                                return false;
                            }
                        }
                        None => {
                            // Set the mapping for this number.
                            letter_map[number] = Some(letter);
                        }
                    }
                } else if number == 1 {
                    // For number 1, ensure all occurrences map to different letters
                    if used_letters_for_1.contains(&letter) {
                        return false;
                    }
                    used_letters_for_1.push(letter);
                } else {
                    // For other numbers, check if the letter is in the possible letters
                    if !possible_letters[number].contains(&letter) {
                        return false;
                    }
                }
            }

            true
        });
    }
}

fn update_possible_letters(input_words: &[Word], possible_letters: &mut Vec<Vec<char>>) {
    // Initialize possible letters for each number as empty
    for letters in possible_letters.iter_mut() {
        letters.clear();
    }

    // Iterate through each number
    for number in 0..10 {
        let mut all_letters_for_number: Vec<char> = Vec::new();

        // Collect letters for this number from each word
        for word in input_words {
            for (index, &num) in word.numbers.iter().enumerate() {
                if num as usize == number {
                    // Collect all letters that appear at this index in possible words
                    let letters_at_index: Vec<char> = word
                        .possible_words
                        .iter()
                        .map(|word| word.chars().nth(index).unwrap())
                        .collect();

                    all_letters_for_number.extend(letters_at_index);
                }
            }
        }

        // Deduplicate letters for this number
        all_letters_for_number.sort_unstable();
        all_letters_for_number.dedup();

        // Assign to possible_letters
        possible_letters[number] = all_letters_for_number;
    }
}

fn further_refine_possible_words(input_words: &mut Vec<Word>, possible_letters: &Vec<Vec<char>>) {
    for word in input_words.iter_mut() {
        word.possible_words.retain(|possible_word| {
            // Remove any words that have a letter that is not in the possible letters for that number
            for (number, letter) in word.numbers.iter().zip(possible_word.chars()) {
                let number = *number as usize;
                if !possible_letters[number].contains(&letter) {
                    return false;
                }
            }
            true
        });
    }
}

pub fn solve(input: &str, wordlist: &str) -> i64 {
    // Input example: 58 2741 3335
    // Seperate the input into a vector of vectors of numbers
    let input_numbers: Vec<Vec<u8>> = input
        .split_whitespace()
        .map(|word| {
            word.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    // Collect all available words into a vector
    let mut possible_words: Vec<&str> = wordlist.lines().collect();
    println!("Starting: {:?}", possible_words.len());

    // Filter words from the wordlist that have a length in the word_lengths vector
    filter_words_globally(&input_numbers, &mut possible_words);
    println!("Filtered words count: {}", possible_words.len());

    // For every number 0-9 create a list of possible letters (initially all 26)
    let mut possible_letters: Vec<Vec<char>> =
        vec!["abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>(); 10];

    // Create a word struct for each input word, and add the possible words to the struct with the same length as the input word
    let mut input_words: Vec<Word> = Vec::new();
    for input_word in input_numbers.iter() {
        let mut word = Word {
            numbers: input_word.clone(),
            possible_words: Vec::new(),
        };
        word.possible_words = possible_words
            .iter()
            .filter(|word| word.len() == input_word.len())
            .map(|word| word.to_string())
            .collect();
        input_words.push(word);
    }

    // Count how many times each number appears in the input
    let mut number_counts: Vec<u8> = vec![0; 10];
    for input_word in input_numbers.iter() {
        for number in input_word.iter() {
            number_counts[*number as usize] += 1;
        }
    }
    println!("Number counts:");
    for (i, number_count) in number_counts.iter().enumerate() {
        println!("{}: {}", i, number_count);
    }

    // Get the number of letters for each number (if the number 3 appears 9 times, then there the number 3 can correspond to 3 different letters)
    // If the number 7 appears 7 times, then there is only 1 letter the number 7 can correspond to
    let mut number_letter_counts: Vec<u8> = vec![0; 10];
    for (i, number_count) in number_counts.iter().enumerate() {
        if i == 0 {
            continue;
        }
        number_letter_counts[i] = *number_count / (i as u8);
    }

    println!("Number of letters per number counts:");
    for (i, number_letter_count) in number_letter_counts.iter().enumerate() {
        println!("{}: {}", i, number_letter_count);
    }
    println!();

    // Print number of possible words for each input word
    println!("Number of possible words:");
    for word in input_words.iter() {
        println!(
            "{}: {}",
            word.numbers
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(""),
            word.possible_words.len()
        );
    }

    refine_possible_words(&mut input_words, &mut possible_letters);

    // Update the possible letters for each number based on the possible words
    update_possible_letters(&input_words, &mut possible_letters);

    // Print all possible letters for each number
    for (i, possible_letter) in possible_letters.iter().enumerate() {
        println!("{}: {:?}", i, possible_letter);
    }
    println!();

    // Print number of possible words for each input word
    println!("Number of possible words:");
    for word in input_words.iter() {
        println!(
            "{}: {}",
            word.numbers
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(""),
            word.possible_words.len()
        );
    }

    further_refine_possible_words(&mut input_words, &possible_letters);

    // Print number of possible words for each input word
    println!("\nNumber of possible words:");
    for word in input_words.iter() {
        println!(
            "{}: {}",
            word.numbers
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(""),
            word.possible_words.len()
        );
    }

    0
}
