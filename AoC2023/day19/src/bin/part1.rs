use std::{collections::HashMap, str::FromStr};

struct Rule {
    rating: String,
    dst: String,
    cond: Box<dyn Fn(i32) -> bool>,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cond, dst) = s.split_once(':').ok_or("Expected :")?;
        let cond_fn = if cond.contains('<') {
            i32::lt
        } else if cond.contains('>') {
            i32::gt
        } else {
            unreachable!("Expected either < or >")
        };
        let (rating, cond_val) = cond.split_once(['<', '>']).ok_or("Expected > or <")?;
        let cond_val = cond_val.parse().map_err(|_| "Expected number")?;
        let cond = move |rating| cond_fn(&rating, &cond_val);
        Ok(Self {
            rating: rating.to_string(),
            cond: Box::new(cond),
            dst: dst.to_string(),
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

struct Part {
    ratings: HashMap<String, i32>,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let ratings = s
            .strip_prefix('{')
            .and_then(|s| s.strip_suffix('}'))
            .ok_or("Expected { and }")?
            .split(',');
        for rating in ratings {
            let (rating, value) = rating.split_once('=').ok_or("Expected =")?;
            map.insert(
                rating.to_string(),
                value.parse().map_err(|_| "Expected number")?,
            );
        }
        Ok(Self { ratings: map })
    }
}

impl Part {
    fn get_dst(&self, workflow: &WorkFlow) -> String {
        for rule in &workflow.rules {
            let rating_value = self.ratings[&rule.rating];
            if (rule.cond)(rating_value) {
                return rule.dst.clone();
            }
        }
        return workflow.fallback_dst.clone();
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows: HashMap<_, _> = workflows
        .lines()
        .map(|l| WorkFlow::from_str(l).unwrap())
        .collect();
    let parts: Vec<_> = parts.lines().map(|l| Part::from_str(l).unwrap()).collect();
    let sum: i32 = parts
        .iter()
        .filter(|&p| is_part_accepted(p, "in", &workflows))
        .map(|p| p.ratings.values().sum::<i32>())
        .sum();
    println!("{sum}");
}

fn is_part_accepted(
    part: &Part,
    workflow_name: &str,
    workflows: &HashMap<String, WorkFlow>,
) -> bool {
    let workflow = &workflows[workflow_name];
    let dst = part.get_dst(workflow);
    if dst == "A" {
        true
    } else if dst == "R" {
        false
    } else {
        is_part_accepted(part, &dst, workflows)
    }
}
