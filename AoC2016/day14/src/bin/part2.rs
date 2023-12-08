use md5::compute;

struct Hasher {
    salt: &'static str,
    cache: Vec<String>,
}

impl Hasher {
    fn stretched_md5(&mut self, index: i64) -> String {
        let cached = self.cache.get(index as usize);
        if cached.is_some() {
            cached.unwrap().clone()
        } else {
            let hash = (1..=2017).fold(format!("{}{index}", self.salt), |acc, _| {
                format!("{:?}", compute(acc))
            });
            assert_eq!(index as usize, self.cache.len());
            self.cache.push(hash.clone());
            hash
        }
    }
}

fn main() {
    let mut hasher = Hasher {
        salt: "cuanljph",
        cache: Vec::new(),
    };
    let mut index: i64 = 0;
    let mut key_count = 0;
    while key_count != 64 {
        // println!("{index}");
        let hash = hasher.stretched_md5(index);
        if let Some(c) = has_n(&hash, 3, None) {
            for i in (index + 1)..=(index + 1000) {
                let hash = hasher.stretched_md5(i);
                if has_n(&hash, 5, Some(c)).is_some() {
                    key_count += 1;
                    break;
                }
            }
        }

        index += 1;
    }

    println!("{}", index - 1);
}

fn has_n(s: &str, n: usize, c: Option<char>) -> Option<char> {
    for i in 0..(s.len() - n + 1) {
        let c = c.unwrap_or(s.chars().nth(i).unwrap());
        if s.chars().skip(i).take(n).all(|c1| c1 == c) {
            return Some(c);
        }
    }
    None
}

// fn stretched_md5(s: String) -> String {
//     (1..=2017).fold(s, |acc, _| format!("{:?}", compute(acc)))
// }
