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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        for i in 1..args.len() {
            match &args[i][..] {
                "dna" => dna(),
                "rna" => rna(),
                _ => ()
            };
        }
    }
}
