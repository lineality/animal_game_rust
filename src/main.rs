use rand::Rng;
use std::collections::HashSet;
use std::io::{self, Write};

fn animal_or_not(animal_done_set: &mut HashSet<String>) -> Result<Option<&'static str>, io::Error> {
    // Clear screen with newlines
    println!("{}", "\n".repeat(50));
    
    println!("So far: {:?}\n", animal_done_set);

    // List of three-letter animal names
    const THREE_LETTER_ANIMAL_NAMES: [&str; 16] = [
        "pup", "dog", "cat", "rat", "fox",
        "hen", "bug", "ant", "fly", "pig",
        "bat", "cow", "hog", "ape", "owl",
        "bee",
    ];

    let animals_set: HashSet<_> = THREE_LETTER_ANIMAL_NAMES.iter().map(|&s| s.to_string()).collect();
    
    if &animals_set == animal_done_set {
        println!("All Done!");
        return Ok(Some("All Done!"));
    }

    let mut rng = rand::thread_rng();

    // 25% chance to return a real animal name
    if rng.gen::<f64>() < 0.25 {
        let available_animals: Vec<_> = THREE_LETTER_ANIMAL_NAMES
            .iter()
            .map(|&s| s.to_string())
            .filter(|animal| !animal_done_set.contains(animal))
            .collect();

        if !available_animals.is_empty() {
            let word = available_animals[rng.gen_range(0..available_animals.len())].clone();
            animal_done_set.insert(word.clone());
            println!("{}\n", word);
            return Ok(None);
        }
    }

    // Create lists of letters from each position
    let mut first_letters: HashSet<char> = THREE_LETTER_ANIMAL_NAMES.iter()
        .map(|name| name.chars().next().unwrap())
        .collect();
    let mut second_letters: HashSet<char> = THREE_LETTER_ANIMAL_NAMES.iter()
        .map(|name| name.chars().nth(1).unwrap())
        .collect();
    let mut third_letters: HashSet<char> = THREE_LETTER_ANIMAL_NAMES.iter()
        .map(|name| name.chars().nth(2).unwrap())
        .collect();

    // Add extra letters
    let extra_first = ['t', 'l', 'b', 'p', 's'];
    first_letters.extend(extra_first);

    // Filter second_letters to only include vowels
    let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();
    second_letters = second_letters.intersection(&vowels).cloned().collect();
    
    // If no vowels in second_letters, add them all
    if second_letters.is_empty() {
        second_letters = vowels;
    }

    // Convert sets to vectors for random selection
    let first_letters: Vec<char> = first_letters.into_iter().collect();
    let second_letters: Vec<char> = second_letters.into_iter().collect();
    let third_letters: Vec<char> = third_letters.into_iter().collect();

    // Generate random combination
    let word = format!("{}{}{}",
        first_letters[rng.gen_range(0..first_letters.len())],
        second_letters[rng.gen_range(0..second_letters.len())],
        third_letters[rng.gen_range(0..third_letters.len())]
    );

    // Check if it's a real animal name and add to set if it is
    if THREE_LETTER_ANIMAL_NAMES.contains(&word.as_str()) {
        animal_done_set.insert(word.clone());
    }

    println!("{}\n", word);
    
    Ok(None)
}

fn main() -> io::Result<()> {
    let mut done_set = HashSet::new();
    
    loop {
        match animal_or_not(&mut done_set)? {
            Some("All Done!") => break,
            _ => {
                print!("Press Enter to continue (or type 'exit' to quit): ");
                io::stdout().flush()?;
                
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                if input.trim() == "exit" {
                    println!("OK!");
                    break;
                }
            }
        }
    }
    
    Ok(())
}
