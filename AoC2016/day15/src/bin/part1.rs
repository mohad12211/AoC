fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum.rem_euclid(prod))
}

fn main() {
    let input = include_str!("input.txt");
    let mut modulii: Vec<i64> = Vec::new();
    let mut residues: Vec<i64> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let (_, line) = line.split_at(12);
        let (period, _) = line.split_once(' ').unwrap();
        let (_, positiion) = line.strip_suffix('.').unwrap().rsplit_once(' ').unwrap();
        modulii.push(period.parse().unwrap());
        residues.push(-(positiion.parse::<i64>().unwrap() + i as i64 + 1));
    }

    match chinese_remainder(&residues, &modulii) {
        Some(sol) => println!("{}", sol),
        None => println!("modulii not pairwise coprime"),
    }
}
