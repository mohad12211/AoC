use std::{cmp::Ordering, collections::HashMap, ops::RangeInclusive, str::FromStr};

struct Condition {
    category: usize,
    value: i64,
    ordering: Ordering,
}

struct Rule {
    dst: String,
    cond: Condition,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cond, dst) = s.split_once(':').ok_or("Expected :")?;
        let (category, cond_val) = cond.split_once(['<', '>']).ok_or("Expected > or <")?;
        let value = cond_val.parse().map_err(|_| "Expected number")?;
        let ordering = if cond.contains('<') {
            Ordering::Less
        } else if cond.contains('>') {
            Ordering::Greater
        } else {
            unreachable!("Expected either < or >")
        };
        let category = match category {
            "x" => 0,
            "m" => 1,
            "a" => 2,
            "s" => 3,
            _ => unreachable!(),
        };
        Ok(Self {
            dst: dst.to_string(),
            cond: Condition {
                category,
                value,
                ordering,
            },
        })
    }
}

struct WorkFlow {
    rules: Vec<Rule>,
    fallback_dst: String,
}

impl WorkFlow {
    fn from_str(s: &str) -> Result<(String, WorkFlow), String> {
        let (name, rules) = s
            .strip_suffix('}')
            .and_then(|s| s.split_once('{'))
            .ok_or("Expected {")?;
        let mut rules = rules.split(',').collect::<Vec<_>>();
        let fallback_dst = rules.pop().ok_or("Expected at least one rule")?.to_string();
        let rules: Vec<_> = rules
            .into_iter()
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok((
            name.to_string(),
            Self {
                rules,
                fallback_dst,
            },
        ))
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let (workflows, _) = input.split_once("\n\n").unwrap();
    let workflows: HashMap<_, _> = workflows
        .lines()
        .map(|l| WorkFlow::from_str(l).unwrap())
        .collect();
    let ranges = [(1..=4000), (1..=4000), (1..=4000), (1..=4000)];
    let combinations = get_accepted_combinations("in", &workflows, ranges);
    println!("{combinations}");
}

fn get_accepted_combinations(
    workflow_name: &str,
    workflows: &HashMap<String, WorkFlow>,
    ranges: [RangeInclusive<i64>; 4],
) -> i64 {
    if workflow_name == "R" || ranges.iter().any(|r| r.is_empty()) {
        return 0;
    }
    if workflow_name == "A" {
        return ranges.iter().map(|r| r.end() - r.start() + 1).product();
    }

    let workflow = &workflows[workflow_name];
    let mut ranges = ranges;
    let mut combinations = 0;

    for rule in &workflow.rules {
        let (inside_condition, rest) = split_range_at(
            ranges[rule.cond.category].clone(),
            rule.cond.value,
            rule.cond.ordering,
        );
        let mut rule_ranges = ranges.clone();
        let mut next_ranges = ranges.clone();
        next_ranges[rule.cond.category] = rest;
        rule_ranges[rule.cond.category] = inside_condition;
        combinations += get_accepted_combinations(&rule.dst, workflows, rule_ranges);
        ranges = next_ranges;
    }
    combinations += get_accepted_combinations(&workflow.fallback_dst, workflows, ranges);

    return combinations;
}

fn split_range_at(
    range: RangeInclusive<i64>,
    split_value: i64,
    ord: Ordering,
) -> (RangeInclusive<i64>, RangeInclusive<i64>) {
    match ord {
        Ordering::Less => {
            if split_value < *range.start() {
                return (RangeInclusive::new(0, 0), range);
            } else if split_value > *range.end() {
                return (range, RangeInclusive::new(0, 0));
            } else {
                return (
                    *range.start()..=(split_value - 1),
                    (split_value..=*range.end()),
                );
            }
        }
        Ordering::Greater => {
            if split_value < *range.start() {
                return (range, RangeInclusive::new(0, 0));
            } else if split_value > *range.end() {
                return (RangeInclusive::new(0, 0), range);
            } else {
                return (
                    ((split_value + 1)..=*range.end()),
                    *range.start()..=(split_value),
                );
            }
        }
        Ordering::Equal => unreachable!(),
    }
}
