use std::{fs::File, io::{BufReader, Lines}};


pub fn run(lines:Lines<BufReader<File>>) {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    let mut stable = 0;
    
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        reports.push(line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>());
    }

    'reports: for report in reports {
        let reverse = report[0] > report[1];

        let indices: Box<dyn Iterator<Item = usize>> = if reverse {
            Box::new((0..report.len()).rev())
        } else {
            Box::new(0..report.len())
        };
        
        for idx in indices {
            let prev_idx = if reverse {
                idx + 1
            } else {
                if idx == 0 {
                    continue;
                }
                idx - 1
            };
            let prev = match report.get(prev_idx) {
                Some(n) => *n,
                None => continue
            };
            if prev == report[idx] || (report[idx] - prev).abs() > 3 || prev > report[idx] {
                println!("Unstable: {:?}", report);
                continue 'reports;
            }            
        }

        println!("Stable: {:?}", report);
        stable += 1;
    };

    println!("Part 1 - Stable: {}", stable);

}