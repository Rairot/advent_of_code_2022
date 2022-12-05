use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    const TOP_POSITIONS: usize = 3;

    let lines = read_lines(".//src//input.txt").unwrap();
    let mut top_scores: Vec<usize> = vec![0; TOP_POSITIONS];

    for line_result in lines {
        let mut current_score = 0;

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
