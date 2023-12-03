const INPUT: &str = include_str!("../../inputs/input.txt");

fn main() {
    let weights: Vec<i64> = INPUT.lines().map(|x| x.parse().unwrap()).collect();
    let mut best: Vec<i64> = Vec::new();
    // 4 for part 2, 3 for part 1
    let group_weight = weights.iter().sum::<i64>() / 4;
    let mut max_len = usize::max_value();
    let mut max_product = i64::max_value();

    for w in &weights {
        search(
            weights.iter().filter(|x| x > &w).copied().collect(),
            vec![*w],
            group_weight,
            &mut best,
            &mut max_len,
            &mut max_product,
        );
    }

    println!("{:?}", max_product);
}

fn search(
    unused: Vec<i64>,
    current: Vec<i64>,
    group_weight: i64,
    best: &mut Vec<i64>,
    max_len: &mut usize,
    max_product: &mut i64,
) {
    if current.len() > *max_len {
        return;
    }

    let sum = current.iter().sum::<i64>();

    if sum == group_weight {
        let product = current.iter().product::<i64>();
        if current.len() < *max_len {
            *max_len = current.len();
            *max_product = product;
            *best = current;
        } else if current.len() == *max_len && product < *max_product {
            *best = current;
            *max_product = product;
        }
        return;
    } else if sum > group_weight {
        return;
    }

    if unused.is_empty() {
        return;
    }
    for w in &unused {
        search(
            unused.iter().filter(|x| x > &w).copied().collect(),
            {
                let mut v = current.clone();
                v.push(*w);
                v
            },
            group_weight,
            best,
            max_len,
            max_product,
        );
    }
}
