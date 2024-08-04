use std::str::FromStr;
use super::types::City;

/*

    1) City следует за City
    2) City по порядку Value
    3) City на дистанции Value
    4) City на времени Value
    5) City на дистанции от City Value
    6) City на дистанции от City Value

*/

#[derive(Debug, PartialEq)]
enum Constraint {
    Follows(City, City),
    Precedes(City, City),
    InOrder(Vec<City>),
    And(Box<Constraint>, Box<Constraint>),
    Or(Box<Constraint>, Box<Constraint>),
}

#[derive(Debug, PartialEq)]
enum Action {
    Exclude,
    Penalty(i32),
    Reward(i32),
}

#[derive(Debug)]
pub struct Rule {
    constraint: Constraint,
    action: Action,
}

#[derive(Debug)]
pub struct ParseRuleError;

impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('%').map(str::trim).collect();
        if parts.len() != 2 {
            return Err(ParseRuleError);
        }

        let constraint_str = parts[0];
        let action_str = parts[1];

        let constraint = parse_constraint(constraint_str)?;

        let action = if action_str.contains("исключить") {
            Action::Exclude
        } else if action_str.contains("штраф") {
            let penalty: i32 = action_str
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .next()
                .ok_or(ParseRuleError)?;
            Action::Penalty(penalty)
        } else if action_str.contains("поощрение") {
            let reward: i32 = action_str
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .next()
                .ok_or(ParseRuleError)?;
            Action::Reward(reward)
        } else {
            return Err(ParseRuleError);
        };

        Ok(Rule { constraint, action })
    }
}

fn parse_constraint(s: &str) -> Result<Constraint, ParseRuleError> {
    if s.contains(" и ") {
        let parts: Vec<&str> = s.split(" и ").collect();
        if parts.len() != 2 {
            return Err(ParseRuleError);
        }
        let left = parse_constraint(parts[0])?;
        let right = parse_constraint(parts[1])?;
        return Ok(Constraint::And(Box::new(left), Box::new(right)));
    }

    if s.contains(" или ") {
        let parts: Vec<&str> = s.split(" или ").collect();
        if parts.len() != 2 {
            return Err(ParseRuleError);
        }
        let left = parse_constraint(parts[0])?;
        let right = parse_constraint(parts[1])?;
        return Ok(Constraint::Or(Box::new(left), Box::new(right)));
    }

    if s.contains("следует за") {
        let nums: Vec<City> = s
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if nums.len() != 2 {
            return Err(ParseRuleError);
        }
        Ok(Constraint::Follows(nums[0], nums[1]))
    } else if s.contains("предшествует") {
        let nums: Vec<City> = s
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if nums.len() != 2 {
            return Err(ParseRuleError);
        }
        Ok(Constraint::Precedes(nums[0], nums[1]))
    } else if s.contains("по порядку") {
        let nums: Vec<City> = s
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if nums.is_empty() {
            return Err(ParseRuleError);
        }
        Ok(Constraint::InOrder(nums))
    } else {
        Err(ParseRuleError)
    }
}

pub fn parse_rules(rules: Vec<&str>) -> Vec<Rule> {
    rules.iter().filter_map(|&rule| rule.parse().ok()).collect()
}

pub fn apply_rules(path: &Vec<City>, rules: &Vec<Rule>) -> Option<i32> {
    let mut total_score = 0;

    for rule in rules {
        if !check_constraint(&rule.constraint, path) {
            match rule.action {
                Action::Exclude => return None,
                Action::Penalty(p) => total_score -= p,
                Action::Reward(r) => total_score += r,
            }
        }
    }

    Some(total_score)
}

fn check_constraint(constraint: &Constraint, path: &Vec<City>) -> bool {
    match constraint {
        Constraint::Follows(a, b) => {
            path.iter().position(|&x| x == *a) > path.iter().position(|&x| x == *b)
        }
        Constraint::Precedes(a, b) => {
            path.iter().position(|&x| x == *a) < path.iter().position(|&x| x == *b)
        }
        Constraint::InOrder(order) => {
            let mut last_pos = 0;
            for &city in order {
                if let Some(pos) = path.iter().position(|&x| x == city) {
                    if pos < last_pos {
                        return false;
                    }
                    last_pos = pos;
                } else {
                    return false;
                }
            }
            true
        }
        Constraint::And(left, right) => {
            check_constraint(left, path) && check_constraint(right, path)
        }
        Constraint::Or(left, right) => {
            check_constraint(left, path) || check_constraint(right, path)
        }
    }
}
