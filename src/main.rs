use std::fs::File;
use std::io::prelude::*;

fn convert(directoriy: std::path::PathBuf) -> ([usize; 5], String) {
    println!(
        "{}",
        directoriy.clone().into_os_string().into_string().unwrap()
    );
    let filename = format!("{}", directoriy.file_name().unwrap().to_str().unwrap());
    let paras = directoriy
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split('_')
        .collect::<Vec<_>>();
    (
        [
            paras[0].parse::<usize>().unwrap(),
            paras[1].parse::<usize>().unwrap(),
            paras[2].parse::<usize>().unwrap(),
            paras[3].parse::<usize>().unwrap(),
            paras[4].parse::<usize>().unwrap(),
        ],
        filename,
    )
}

fn parse(
    mut element: std::str::SplitWhitespace,
) -> (
    usize,
    usize,
    usize,
    usize,
    f64,
    usize,
    f64,
    f64,
    usize,
    usize,
) {
    element.next();
    element.next();
    element.next();
    println!();
    let oom = element.next().unwrap();
    let oom = oom
        .chars()
        .take(oom.len() - 1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    println!("oom:: {}", oom);
    element.next();
    let opt = element.next().unwrap();
    let opt = opt
        .chars()
        .take(opt.len() - 1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    println!("opt:: {}", opt);
    element.next();
    let ife = element.next().unwrap();
    let ife = ife
        .chars()
        .take(ife.len() - 1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    println!("ife:: {}", ife);
    element.next();
    element.next();
    element.next();
    element.next();
    let avr = element.next().unwrap().parse::<usize>().unwrap();
    println!("avr:: {}", avr);
    element.next();
    element.next();
    let re = element.next().unwrap().parse::<f64>().unwrap();
    println!("re:: {}", re);
    element.next();
    element.next();
    let max = element.next().unwrap();
    let max = max
        .chars()
        .take(max.len() - 3)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    println!("max:: {}", max);
    element.next();
    let gap_l = element.next().unwrap();
    let gap_l = gap_l
        .chars()
        .take(gap_l.len() - 1)
        .collect::<String>()
        .parse::<f64>()
        .unwrap();
    println!("gap_l:: {}", gap_l);
    element.next();
    let gap_u = element.next().unwrap();
    let gap_u = gap_u
        .chars()
        .take(gap_u.len() - 1)
        .collect::<String>()
        .parse::<f64>()
        .unwrap();
    println!("gap_u:: {}", gap_u);
    element.next();
    let node = element.next().unwrap();
    let node = node
        .chars()
        .take(node.len() - 1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    println!("node:: {}", node);
    element.next();
    let cached = element.next().unwrap().parse::<usize>().unwrap();
    println!("cached:: {}", cached);
    println!();
    (oom, opt, ife, avr, re, max, gap_l, gap_u, node, cached)
}

// run `cargo run --release DIRECTORY CASES`
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("invalid number of arguments: {}", args.len());
        println!("{:?}", args);
        std::process::exit(1);
    }
    let mut directories: Vec<std::path::PathBuf> = Vec::new();
    let mut files = Vec::new();
    for item in std::fs::read_dir(format!("../{}", args[1]))? {
        directories.push(item?.path());
        files.push(convert(directories[directories.len() - 1].clone()));
    }
    files.sort();

    let mut buf = String::new();
    for (i, (params, filename)) in files.into_iter().enumerate() {
        // if filename == "6_5_75_35_30_x100"{
        //     continue;
        // }
        // if filename == "6_4_60_25_30_x100"{
        //     continue;
        // }
        // println!("{:?}", params);
        // println!("{}", filename);
        buf.push_str(&format!(
            "  {} & {} & {} & {} & {} & ",
            params[0], params[1], params[2], params[3], params[4]
        ));
        let mut file = File::open(&format!("../{}/{}/benchmark", args[1], filename))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let element = contents.split_whitespace();
        let (oom, opt, ife, avr, re, max, gap_l, gap_u, node, cached) = parse(element);
        buf.push_str(&format!(
            "{} & {} & {} & {:.2} & {:.2} & {:.2} & {:.3} & {:.3} & {:.1} & {:.1} \\\\ \\hline",
            oom,
            opt,
            ife,
            avr as f64 / 1000.0,
            max as f64 / 1000.0,
            re,
            gap_l,
            gap_u,
            node as f64 / 10000.0,
            cached as f64 / 10000.0
        ));
        if i % 6 == 5 {
            buf.push_str("\\hline\n")
        } else {
            buf.push('\n');
        }
    }
    let mut file = File::create(&format!(
        "target/table_{}",
        std::path::PathBuf::from(args[1].chars().collect::<String>())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
    ))?;
    file.write_all(buf.as_bytes())
}
