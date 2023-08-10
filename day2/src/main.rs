use std::fs::read_to_string;

fn main() {
    let bind = read_to_string("input.txt").unwrap();
    let iter = bind.split("\n").into_iter();
    let res = iter.map(|a| {
        match a {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1, 
            "B Y" => 5, 
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => 0
    
        } 
    } ); 

    let ints_for_good_guide: Vec<usize> = res.collect();
    let sum_of_good_guide:usize = ints_for_good_guide.iter().sum();

    println!("{:?}", sum_of_good_guide);

    let bind2 = read_to_string("input.txt").unwrap();
    let iter2 = bind2.split("\n").into_iter();

    let res2 = iter2.map(|a| {
            match a {
                "A X" => 3,
                "A Y" => 4,
                "A Z" => 8,
                "B X" => 1, 
                "B Y" => 5, 
                "B Z" => 9,
                "C X" => 2,
                "C Y" => 6,
                "C Z" => 7,
                _ => 0
        
            } 
        } );

    let ints_bad: Vec<usize> = res2.collect();
    let sum_of_bad: usize = ints_bad.iter().sum();
    
    println!("{:?}", sum_of_bad)
}

