use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

fn main() {
    let file = File::open("input.txt");
    let reader = BufReader::new(file.unwrap());
    
    //let results = Vec::new();
    
let mut sum = 0;

    for line in reader.lines() {
        let middle = line.as_ref().unwrap().len()/2;
        let (first_half,second_half) = line.as_ref().unwrap().split_at(middle);
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut map2: HashMap<char, usize> = HashMap::new();
        first_half.chars().for_each(|c| {
           *map.entry(c).or_insert_with(|| 1) += 1;
        });
        second_half.chars().for_each(|c| {
           *map2.entry(c).or_insert_with(|| 1) += 1;
        });
        let common_key:Vec<&char> = map.keys().filter_map(|k| {
            if map2.contains_key(k) {
                Some(k)
            }
            else {
                None
            }

        }).collect();
        
        let a = 1;
        let A = 27;

        println!("{:?}",'a' as usize);
        println!("{:?}",'A' as usize);
        
        if *common_key[0] as usize > 96 {
            sum += *common_key[0] as usize - 'a' as usize + a;
        } else {
            sum += *common_key[0] as usize - 'A' as usize + A;
        }
   }

    println!("{:?}", sum);
}
