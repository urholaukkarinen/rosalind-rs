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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        for i in 1..args.len() {
            match &args[i][..] {
                "dna" => dna(),
                "rna" => rna(),
                "revc" => revc(),
                _ => ()
            };
        }
    }
}
