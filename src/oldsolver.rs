pub fn solve(input: &str, wordlist: &str) -> i64 {
    // Collect all available words into a vector
    let mut possible_words: Vec<&str> = wordlist.lines().collect();
    println!("Starting: {:?}", possible_words.len());

    // Collect input words (list of numbers) into a vector
    let input_words: Vec<&str> = input.split_whitespace().collect();

    // Get the number of letters for each word
    let mut word_lengths: Vec<usize> = input_words.iter().map(|word| word.len()).collect();

    // Remove any duplicates
    word_lengths.sort_unstable();
    word_lengths.dedup();
    println!("Word lengths: {:?}", word_lengths);

    // Filter words from the wordlist that have a length in the word_lengths vector
    possible_words.retain(|word| word_lengths.contains(&word.len()));
    println!("Filtered words count: {}", possible_words.len());

    // For every number 0-9 create a list of possible letters (initially all 26)
    let mut possible_letters: Vec<Vec<char>> =
        vec!["abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>(); 10];

    // Set 7 to only be an 'a' or 'b'
    possible_letters[7] = vec!['a', 'e'];

    // Get all words of length 10 that start with an e and 4th letter is an 'a'
    let mut possible_words: Vec<&str> = possible_words
        .iter()
        .filter(|word| {
            word.len() == 10 && word.starts_with('a') && word.chars().nth(3).unwrap() == 'e'
        })
        .copied()
        .collect();

    // Remove any words with more than 1 a or 3
    possible_words.retain(|word| {
        let mut a_count = 0;
        let mut e_count = 0;
        for c in word.chars() {
            if c == 'a' {
                a_count += 1;
            } else if c == 'e' {
                e_count += 1;
            }
        }
        a_count == 1 && e_count == 1
    });

    println!("Filtered words count: {}", possible_words.len());
    for word in possible_words.iter() {
        println!("{}", word);
    }

    0
}
