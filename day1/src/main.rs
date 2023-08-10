use std::fs::read_to_string;

fn main() {
    let mut calories = Vec::new();

    let mut cal = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
       if line == "" {
            calories.push(cal);
            cal = 0;
       } else {
            cal += line.parse::<usize>().unwrap();
       }
    }

    calories.sort_by(|a,b| b.cmp(a));
    println!("{:?}", &calories.iter().take(3).sum::<usize>());
}
