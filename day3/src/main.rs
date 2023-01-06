use utils::read_lines;

fn find_common_item(list_a: &str, list_b: &str, list_c: Option<&str>) -> Option<char> {
    for character in list_a.chars() {
        if list_c.is_none() {
            if list_b.contains(character) {
                return Some(character);
            }
        } else {
            if list_b.contains(character) && list_c.unwrap().contains(character) {
                return Some(character);
            }
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
    let part1_lines = read_lines(".//src//part1_input.txt").unwrap();
    let part2_lines = read_lines(".//src//part2_input.txt").unwrap();

    let mut part1_results: Vec<char> = Vec::new();
    let mut part2_results: Vec<char> = Vec::new();

    let mut group_buffer: Vec<String> = Vec::new();

    for line_result in part1_lines {
        if let Ok(line) = line_result {
            let split_point = line.len() / 2;
            let compartment_one = line.get(..split_point).expect("Compartment one");
            let compartment_two = line.get(split_point..).expect("Compartment two");

            let result = find_common_item(compartment_one, compartment_two, None);
            part1_results.push(result.expect("No common item found"));
        }
    }

    for line_result in part2_lines {
        if let Ok(line) = line_result {
            group_buffer.push(line);
            if group_buffer.len() % 3 == 0 {
                let group_result = find_common_item(
                    group_buffer[0].as_str(),
                    group_buffer[1].as_str(),
                    Some(group_buffer[2].as_str()),
                );
                part2_results.push(group_result.expect("No common item found"));
                group_buffer.clear();
            }
        }
    }
    println!(
        "Total score for part 1: {:?}",
        calculate_score(part1_results)
    );
    println!(
        "Total score for part 2: {:?}",
        calculate_score(part2_results)
    );
}
