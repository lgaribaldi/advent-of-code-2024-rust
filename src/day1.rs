use std::{collections::HashMap, fs::File, io::{BufReader, Lines}};


pub fn run(lines:Lines<BufReader<File>>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut diff = 0;
    let mut sim = 0;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        let mut chunks = line.split("   ");
        left.push(chunks.next().unwrap().parse::<i32>().unwrap());
        right.push(chunks.next().unwrap().parse::<i32>().unwrap());

    }

    // part 1
    left.sort();
    right.sort();
    let mut right_iter = right.clone().into_iter();
    left.clone().into_iter().for_each(|l| {
        let r = right_iter.next().unwrap();
        diff += (r - l).abs();
    });

    //part 2
    let mut map: HashMap<i32, i32> = HashMap::new();
    right.into_iter().for_each(|r| {
        let curr = map.get(&r).unwrap_or(&0);
        map.insert(r, curr + 1);
    });

    left.into_iter().for_each(|l| {
        let times = map.get(&l).unwrap_or(&0);
        sim += l * times;
    });

    println!("Part 1 - Difference: {}", diff);
    println!("Part 2 - Similarity: {}", sim);

}