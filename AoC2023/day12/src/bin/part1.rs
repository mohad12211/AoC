use std::str::FromStr;

#[derive(Clone)]
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

        Ok(Self {
            springs: springs.to_string(),
            groups: groups
                .split(',')
                .map(|g| g.parse())
                .collect::<Result<_, _>>()
                .map_err(|_| "groups should be numbers")?,
        })
    }
}

impl Record {
    fn is_valid(&self) -> bool {
        assert!(!self.springs.contains('?'));
        let grouped_springs = self
            .springs
            .split('.')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        self.groups.len() == grouped_springs.len()
            && grouped_springs
                .iter()
                .zip(self.groups.iter())
                .all(|(group, group_count)| group.len() == *group_count)
    }

    fn find_combinations(&self) -> i64 {
        if let Some(index) = self.springs.find('?') {
            let mut c1 = self.clone();
            c1.springs.replace_range(index..(index + 1), ".");
            let mut c2 = self.clone();
            c2.springs.replace_range(index..(index + 1), "#");
            return c1.find_combinations() + c2.find_combinations();
        } else {
            return self.is_valid() as i64;
        }
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let records: Vec<_> = input
        .lines()
        .map(|l| Record::from_str(l).unwrap())
        .collect();

    let sum: i64 = records.iter().map(|r| r.find_combinations()).sum();
    println!("{sum}");
}
