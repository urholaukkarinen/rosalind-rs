use std::env;
use std::collections::HashMap;
use std::cmp::Ordering::Equal;

macro_rules! solutions(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m : HashMap<&str, &dyn Fn() -> String> = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

fn dna() -> String {
    let dataset = include_str!("rosalind_dna.txt").trim();

    format!("{} {} {} {}",
        dataset.matches('A').count(),
        dataset.matches('C').count(),
        dataset.matches('G').count(),
        dataset.matches('T').count())
}

fn rna() -> String {
    include_str!("rosalind_rna.txt").trim().replace("T", "U")
}

fn revc() -> String {
    include_str!("rosalind_revc.txt").trim().chars().map(|c| match c {
        'A' => 'T',
        'C' => 'G',
        'G' => 'C',
        'T' => 'A',
        _ => c
    }).rev().collect::<String>()
}

fn fib() -> String {
    let vals : Vec<u64> = include_str!("rosalind_fib.txt")
        .trim().split_whitespace()
        .map(|x|x.parse().unwrap()).collect();
    let (n, k) = (vals[0], vals[1]);

    let mut kits : u64 = 1;
    let mut adults : u64 = 0;

    for _ in 0..n-1 {
        let born = adults * k;
        adults += kits;
        kits = born;
    }

    format!("{}", adults + kits)
}

fn gc() -> String {
    include_str!("rosalind_gc.txt").trim().split(">")
    .filter(|x|!x.is_empty())
    .map(|x| {
        let mut split = x.splitn(2, '\n');
        let name = split.next().unwrap();
        let dna = split.next().unwrap().replace('\n', "");
        (name, dna)
    })
    .map(|x| {
        let gc_count = x.1.matches('G').count() + x.1.matches('C').count();
        let percentage = gc_count as f32 / x.1.len() as f32 * 100.0;
        (x.0, percentage)
    })
    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Equal))
    .map(|x| format!("{}\n{}", x.0, x.1))
    .unwrap()
}

fn main() {
    let solutions = solutions!{
        "dna" => &dna,
        "rna" => &rna,
        "revc"=> &revc,
        "fib" => &fib,
        "gc"  => &gc
    };

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        for i in 1..args.len() {
            let name = &args[i][..];
            if let Some(solution) = solutions.get(name) {
                println!("[{}] solution:", name);
                println!("{}\n", solution());
            }
        }
    }
}
