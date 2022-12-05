use utils::read_lines;

fn main() {
    const TOP_POSITIONS: usize = 3;

    let lines = read_lines(".//src//input.txt").unwrap();
    let mut top_scores: Vec<usize> = vec![0; TOP_POSITIONS];
    let mut current_score = 0;

    for line_result in lines {
        if let Ok(line) = line_result {
            if !line.is_empty() {
                current_score += line.parse::<usize>().unwrap();
            } else {
                for i in 0..=TOP_POSITIONS - 1 {
                    if current_score > top_scores[i] {
                        top_scores.insert(i, current_score);

                        top_scores = top_scores[0..TOP_POSITIONS].to_vec();
                        current_score = 0;
                    }
                }
                current_score = 0;
            }
        }
    }

    let answer: usize = top_scores.iter().sum();
    println!("Top {TOP_POSITIONS} elves' sum calories {answer}",);
}
