use utils::read_lines;

fn find_common_item(compartment_one: &str, compartment_two: &str) -> Option<char> {
    for character in compartment_one.chars() {
        if compartment_two.contains(character) {
            return Some(character);
        }
    }
    None
}

fn calculate_score(items: Vec<char>) -> usize {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".char_indices();
    let mut total_score = 0;

    for (priority, character) in alphabet.into_iter() {
        for item in items.iter() {
            if character.eq(&item) {
                total_score += priority + 1;
            }
        }
    }
    total_score
}

fn main() {
    let lines = read_lines(".//src//input.txt").unwrap();

    let mut results: Vec<char> = Vec::new();
    for line_result in lines {
        if let Ok(line) = line_result {
            let split_point = line.len() / 2;
            let compartment_one = line.get(..split_point).expect("Compartment one");
            let compartment_two = line.get(split_point..).expect("Compartment two");

            let result = find_common_item(compartment_one, compartment_two);
            results.push(result.expect("No common item found"));
        }
    }

    println!("Total score: {:?}", calculate_score(results));
}
