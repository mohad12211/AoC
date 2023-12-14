use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let num = input.trim().parse().unwrap();
    let larger_than_num = spiral(num);
    println!("{larger_than_num}");
}

fn get_next((x, y): (i64, i64), map: &HashMap<(i64, i64), usize>) -> usize {
    let mut sum = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }
            sum += map.get(&(x + dx, y + dy)).unwrap_or(&0);
        }
    }
    sum
}

fn spiral(num: usize) -> usize {
    let mut map = HashMap::new();
    map.insert((0, 0), 1);

    let mut i = 1;
    let (mut x, mut y) = (1, 0);
    loop {
        while y < i {
            let next = get_next((x, y), &map);
            if next > num {
                return next;
            }
            map.insert((x, y), next);
            y += 1;
        }
        while x > -i {
            let next = get_next((x, y), &map);
            if next > num {
                return next;
            }
            map.insert((x, y), next);
            x -= 1;
        }
        while y > -i {
            let next = get_next((x, y), &map);
            if next > num {
                return next;
            }
            map.insert((x, y), next);
            y -= 1;
        }
        while x <= i {
            let next = get_next((x, y), &map);
            if next > num {
                return next;
            }
            map.insert((x, y), next);
            x += 1;
        }
        i += 1;
    }
}
