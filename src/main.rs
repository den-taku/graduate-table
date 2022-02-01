use std::fs::File;
use std::io::prelude::*;

fn calc(
    mut values: std::str::SplitWhitespace,
    opt: &mut usize,
    infeasible: &mut usize,
    avr: &mut usize,
    sum: &mut u128,
    max: &mut u128,
    gap_lb: &mut usize,
    gap_ub: &mut usize,
    nodes: &mut u128,
    cached: &mut u128,
    oom: &mut usize,
    fail_count: &mut usize,
) {
    if values.next() == Some("0") {
        // oom
        *oom += 1;
        return;
    }
    if values.next() == Some("0") {
        // time limit
        return;
    }
    *opt += 1;
    if let Some(time) = values.next() {
        let time = time.parse::<u128>().unwrap();
        *sum += time;
        *max = std::cmp::max(*max, time);
    }
    if values.next() == Some("0") {
        // no feasible answer
        *infeasible += 1;
        values.next();
        values.next();
        values.next();
    } else {
        if let Some(relocations) = values.next() {
            *avr += relocations.parse::<usize>().unwrap();
        }
        if let Some(g_lb) = values.next() {
            if g_lb == "None" {
                unreachable!()
            } else {
                let number = g_lb
                    .chars()
                    .take(g_lb.len() - 1)
                    .skip(5)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                *gap_lb += number;
            }
        }
        if let Some(g_ub) = values.next() {
            if g_ub == "None" {
                *fail_count += 1;
            } else {
                let number = g_ub
                    .chars()
                    .take(g_ub.len() - 1)
                    .skip(5)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                *gap_ub += number;
            }
        }
    }
    *nodes += values.next().unwrap().parse::<u128>().unwrap();
    *cached += values.next().unwrap().parse::<u128>().unwrap();
}

// run `cargo run --release DIRECTORY CASES`
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("invalid number of arguments: {}", args.len());
        println!("{:?}", args);
        std::process::exit(1);
    }
    let size = args[2].parse::<usize>().unwrap();
    let mut directories: Vec<std::path::PathBuf> = Vec::new();
    for item in std::fs::read_dir(format!("../results/{}", args[1])).unwrap() {
        directories.push(item.unwrap().path());
        println!("{}", item.unwrap().path());
    }

    let mut opt = 0usize;
    let mut infeasible = 0usize;
    let mut avr = 0usize;
    let mut sum = 0u128;
    let mut max = 0u128;
    let mut gap_lb = 0usize;
    let mut gap_ub = 0usize;
    let mut nodes = 0u128;
    let mut cached = 0u128;
    let mut oom = 0usize;
    let mut fail_count = 0usize;

    for i in 0..size {
        let path = format!("{}/result{}", &args[1], i);
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let values: std::str::SplitWhitespace = contents.split_whitespace();
        for e in values.clone() {
            println!("{:?}", e);
        }
        calc(
            values,
            &mut opt,
            &mut infeasible,
            &mut avr,
            &mut sum,
            &mut max,
            &mut gap_lb,
            &mut gap_ub,
            &mut nodes,
            &mut cached,
            &mut oom,
            &mut fail_count,
        );
    }

    let mut file = File::create(format!("{}/benchmark", args[1]))?;
    let mut buf = String::new();
    buf.push_str(&format!("{} cases\n", size));
    if opt == 0 {
        buf.push_str("opt: 0\n");
    } else {
        let gap_lb_avr = if opt == infeasible {
            0.0
        } else {
            gap_lb as f64 / (opt - infeasible) as f64
        };
        let gap_ub_avr = if opt == infeasible + fail_count {
            0.0
        } else {
            gap_lb as f64 / (opt - infeasible - fail_count) as f64
        };
        buf.push_str(&format!(
            "oom: {}, opt: {}, ife: {}, sum: {} ms, avr: {} ms, re: {} times, max: {}ms, gap_lb: {}, gap_ub: {}, nodes: {}, cached: {}\n",
            oom,
            opt,infeasible,
            sum,
            sum / opt as u128,
            avr as f64 / (opt - infeasible) as f64,
            max,
            gap_lb_avr,
            gap_ub_avr,
            nodes / opt as u128,
            cached / opt as u128,
        ));
    }
    file.write_all(buf.as_bytes())
}