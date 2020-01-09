use std::env;

fn dna() {
    println!("[dna] solution: ");

    let dataset = include_str!("rosalind_dna.txt");

    println!("{} {} {} {}",
        dataset.matches('A').count(),
        dataset.matches('C').count(),
        dataset.matches('G').count(),
        dataset.matches('T').count());
}

fn rna() {
    println!("[rna] solution: ");

    let dataset = include_str!("rosalind_rna.txt");

    println!("{}", dataset.replace("T", "U"));
}

fn revc() {
    println!("[revc] solution: ");

    let dataset = include_str!("rosalind_revc.txt");

    let result : String = dataset.chars().map(|c| match c {
        'A' => 'T',
        'C' => 'G',
        'G' => 'C',
        'T' => 'A',
        _ => c
    }).rev().collect();

    println!("{}", result);
}

fn fib() {
    println!("[fib] solution: ");

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

    println!("{}", adults + kits);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        for i in 1..args.len() {
            match &args[i][..] {
                "dna" => dna(),
                "rna" => rna(),
                "revc" => revc(),
                "fib" => fib(),
                _ => ()
            };
        }
    }
}
