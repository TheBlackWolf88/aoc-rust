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
        let big_a = 27;
        
        if *common_key[0] as usize > 96 {
            sum += *common_key[0] as usize - 'a' as usize + a;
        } else {
            sum += *common_key[0] as usize - 'A' as usize + big_a;
        }
   }

    println!("Part 1: {:?}", sum);

    let file2 = File::open("input.txt");
    let reader2 = BufReader::new(file2.unwrap());
    let mut buffer: Vec<String> = Vec::with_capacity(3);
    let mut sum2 = 0;
    for item in reader2.lines() {
        buffer.push(item.unwrap());
        if buffer.len() == 3 {
            let mut map3: HashMap<char, usize> = HashMap::new();
            let mut map4: HashMap<char, usize> = HashMap::new();
            let mut map5: HashMap<char, usize> = HashMap::new();
            buffer[0].chars().for_each(|c| {
                *map3.entry(c).or_insert_with(|| 1) += 1;
            });
            buffer[1].chars().for_each(|c| {
                *map4.entry(c).or_insert_with(|| 1) += 1;
            });
            buffer[2].chars().for_each(|c| {
                *map5.entry(c).or_insert_with(|| 1) += 1;
            });
            let badge:Vec<&char> = map3.keys().filter_map(|k| {
                if map4.contains_key(k) && map5.contains_key(k) {
                    Some(k)
                }
                else {
                    None
                }

            }).collect();

            let a = 1;
            let big_a = 27;
        
            if *badge[0] as usize > 96 {
                sum2 += *badge[0] as usize - 'a' as usize + a;
            } else {
                sum2 += *badge[0] as usize - 'A' as usize + big_a;
            }
            buffer.clear();
        }
    }

    println!("Part 2: {:?}", sum2);
}
