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

    for mut report in reports {
        
        println!("Report: {:?}", report);
        match check_report(&report) {
            None => {
                println!("Stable");
                stable += 1;
            },
            Some((a,b)) => {
                let mut report_a = report.clone();
                report_a.remove(a);
                if check_report(&report_a).is_none() {
                    println!("Stable");
                    stable += 1;
                } else {
                    let mut report_b = report.clone();
                    report_b.remove(b);
                    if check_report(&report_b).is_none() {
                        println!("Stable");
                        stable += 1;
                    } else {
                        let mut report_c = report.clone();
                        report_c.remove(1);
                        if check_report(&report_c).is_none() {
                            println!("Stable");
                            stable += 1;
                        } else {

                            report.remove(0);
                            if check_report(&report).is_none() {
                                println!("Stable");
                                stable += 1;
                            } else {
                                println!("Unstable");
                            }
                        }
                    }
                }
            }
        }

    };

    println!("Part 2 - Stable: {}", stable);

}

fn check_report(report: &Vec<i32>) -> Option<(usize, usize)> {
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
            return Some((idx, prev_idx))
        }            
    }
    return None
}