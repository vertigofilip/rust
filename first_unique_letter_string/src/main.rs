use std::collections::HashMap;

fn main() {
    let text = "dsiafdsjgifikofjiodsuIJFvxdZIJGFUzdxhcJSDvhuisDYHdxcijuSDXcDSvidojhdsugfiZHudijdsisdhuaiohsadifuhiua";
    let mut duplicated_letters = 0;
    let window_size = 10;
    let mut counts: HashMap<char, i32> = HashMap::new();
    
    for i in 0..text.len() {
        // Get the character at position i
        if let Some(ch) = text.chars().nth(i) {
            // Increment the count for this character
            *counts.entry(ch).or_insert(0) += 1;
            if *counts.entry(ch).or_insert(0) == 2 {
                duplicated_letters += 1;
            }
        }
        if i >= window_size {
            if let Some(ch) = text.chars().nth(i - window_size) {
                *counts.entry(ch).or_insert(0) -= 1;
                if *counts.entry(ch).or_insert(0) == 1 {
                    duplicated_letters -= 1;
                    if duplicated_letters == 0 {
                        println!("First unique letter window ends on index {}", i);
                        break;
                    }
                }
            }
        }
    }
    
    // Print the character counts
    for (ch, count) in &counts {
        println!("Character '{}' appears {} times", ch, count);
    }
    println!("Duplicated letters: {}", duplicated_letters);
}