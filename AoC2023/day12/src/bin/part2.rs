use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Debug, Hash)]
struct Record {
    springs: String,
    groups: Vec<usize>,
}

impl FromStr for Record {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, groups) = s
            .split_once(' ')
            .ok_or("no space between groups and springs")?;

        let mut springs = (springs.to_string() + "?").repeat(5);
        springs.pop();

        let groups: Vec<usize> = groups
            .split(',')
            .map(|g| g.parse())
            .collect::<Result<_, _>>()
            .map_err(|_| "groups should be numbers")?;

        let groups = groups.repeat(5);

        Ok(Self { springs, groups })
    }
}

fn combinations<'a>(
    springs: &'a [u8],
    groups: &'a [usize],
    cache: &mut HashMap<(&'a [u8], &'a [usize]), i64>,
) -> i64 {
    if springs.is_empty() {
        if groups.is_empty() {
            // if we have no springs and no groups, everything was matched, valid combination
            return 1;
        } else {
            // if we still have groups to match but no springs, it's invalid
            return 0;
        }
    }

    if groups.is_empty() {
        if springs.contains(&b'#') {
            // if we still have springs to match but no groups, invalid combination
            return 0;
        } else {
            // if we have no groups and have no springs left ('.' or '?'), then the rest of '?' can
            // be '.', valid combination
            return 1;
        }
    }

    if let Some(&cached) = cache.get(&(springs, groups)) {
        return cached;
    }

    let mut total_combinations = 0;
    let current_group = groups[0];

    // treat '?' as '.'
    if springs[0] == b'.' || springs[0] == b'?' {
        total_combinations += combinations(&springs[1..], groups, cache);
    }

    // treat ? as '#'
    if springs[0] == b'#' || springs[0] == b'?' {
        // make sure we have enough springs for the group
        // and we can have a block of springs (the next `current_group` springs are not '.')
        if current_group <= springs.len() && !springs[..groups[0]].contains(&b'.') {
            if current_group == springs.len() {
                // if the the springs string matches the group, remove them
                total_combinations +=
                    combinations(&springs[(current_group)..], &groups[1..], cache);
            } else if springs[current_group] != b'#' {
                // if the springs are larger the than group, the next spring after the group
                // should be '?' or '.' to end the springs group, because groups are always
                // separated by at least one operational spring
                total_combinations +=
                    combinations(&springs[(current_group + 1)..], &groups[1..], cache);
            }
        }
    }

    cache.insert((springs, groups), total_combinations);
    total_combinations
}

fn main() {
    // let input = include_str!("input.test.txt");
    let mut cache: HashMap<(&[u8], &[usize]), i64> = HashMap::new();
    let input = include_str!("input.txt");
    let records: Vec<_> = input
        .lines()
        .map(|l| Record::from_str(l).unwrap())
        .collect();
    let sum: i64 = records
        .iter()
        .map(|r| combinations(r.springs.as_bytes(), &r.groups, &mut cache))
        .sum();
    println!("{sum}");
}
