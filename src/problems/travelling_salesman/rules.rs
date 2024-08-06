use std::ops::{Index, RangeInclusive};
use regex::Regex;
use crate::problems::travelling_salesman::types::{City, Matrix, RuleStr};
use crate::problems::travelling_salesman::helpers::calculate_distance;
/*

    1) City следует за City
    2) City по порядку Value
    3) City на дистанции Value
    4) City на времени Value
    5) City на дистанции от City Value
    6) City на дистанции от City Value

*/

#[derive(Debug)]
enum Condition {
    Follows(String, String),
    InOrder(String, u32),
    OnDistance(String, String),
    OnDistanceFromCity(String, String, String),
    OnTime(String, String),
    OnTimeFromCity(String, String, String),
}

enum Range {
    Single(i32),
    FromTo(i32, i32),
    From(i32),
    To(i32),
}


const RANGE_RE: Regex = Regex::new(r"^\[(\d*),(\d*)\]$").unwrap();

fn parse_range(s: String) -> Result<Range, String> {
    if let Ok(num) = s.parse::<i32>() {
        // Если строка - целое число
        Ok(Range::Single(num))
    } else if let Some(captures) = RANGE_RE.captures(s) {
        // Если строка соответствует формату диапазона
        let start = captures.get(1).map(|m| m.as_str()).unwrap_or("");
        let end = captures.get(2).map(|m| m.as_str()).unwrap_or("");

        match (start, end) {
            ("", "") => Err("Пустой диапазон".to_string()),
            (s, "") => Ok(Range::From(s.parse().unwrap())),
            ("", e) => Ok(Range::To(e.parse().unwrap())),
            (s, e) => Ok(Range::FromTo(s.parse().unwrap(), e.parse().unwrap())),
        }
    } else {
        Err("Неправильный формат".to_string())
    }
}

fn follows(city_a: City, city_b: City, cities: &Vec<City>) -> bool {
    cities.index(city_b) - cities.index(city_a) == 1
}

fn in_order(city: City, order: u32, cities: &Vec<City>) -> bool {
    cities.index(city) + 1 == *order
}

fn is_distance_in_range(range: RangeInclusive<usize>, distance_raw: String, cities: &Vec<City>, matrix: &Matrix) -> bool {
    let distance = calculate_distance(matrix, &cities[range].to_vec());

    match parse_range(distance_raw) {
        Range::Single(val) => distance == val as f64,
        Range::From(val) => distance >= val as f64,
        Range::To(val) => distance <= val as f64,
        Range::FromTo(from, to) => distance >= from as f64 && distance <= to as f64
    }
}

fn on_distance(city: City, distance_raw: String, cities: &Vec<City>, matrix: &Matrix) -> bool {
    let city_idx = cities.iter().position(city).unwrap();

    is_distance_in_range(0..=city_idx, distance_raw, cities, matrix)
}

fn on_distance_from_city(city_to: City, city_from: City, distance_raw: String, cities: &Vec<City>, matrix: &Matrix) -> bool {
    let city_to_idx = cities.iter().position(city_to).unwrap();
    let city_from_idx = cities.iter().position(city_from).unwrap();
    let cities_range = if city_from_idx > city_to_idx {
        city_to_idx..=city_from_idx
    } else {
        city_from_idx..=city_to_idx
    };

    is_distance_in_range(cities_range, distance_raw, cities, matrix)
}

fn parse_conditions(s: &str, cities: Vec<City>, matrix: Matrix) {
    let whitespaces_pattern = Regex::new(r"\s{2,}").unwrap();
    let s = whitespaces_pattern.replace_all(s, " ");
    let operators_pattern = Regex::new(r"\s+(и|или)\s+").unwrap();

    let follows_re = Regex::new(r"(\w+)\s+следует за\s+(\w+)").unwrap();
    let in_order_re = Regex::new(r"(\w+)\s+по порядку\s+(\d+)").unwrap();
    let on_distance_re = Regex::new(r"(\w+)\s+на дистанции\s+(\d+|\[.*?\])").unwrap();
    let on_distance_from_city_re = Regex::new(r"(\w+)\s+на дистанции от\s+(\w+)\s+(\d+|\[.*?\])").unwrap();
    // let on_time_re = Regex::new(r"(\w+)\s+на времени\s+(\d+|\[.*?\])").unwrap();
    // let on_time_from_city_re = Regex::new(r"(\w+)\s+на времени от\s+(\w+)\s+(\d+|\[.*?\])").unwrap();


    let mut condition = s.replace(" и ", " && ").replace(" или ", " || ");
    for part in operators_pattern.replace_all(&*s, "#").split("#") {
        if let Some(cap) = follows_re.captures(part) {
            let city_a = cap[1].parse::<City>().unwrap();
            let city_b = cap[2].parse::<City>().unwrap();

            let result = follows(city_a, city_b, &cities).to_string();
            condition = condition.replace(&*cap[0].to_string(), &*result);
        } else if let Some(cap) = in_order_re.captures(part) {
            let city = cap[1].parse::<City>().unwrap();
            let order = cap[2].parse::<u32>().unwrap();

            let result = in_order(city, order, &cities).to_string();
            condition = condition.replace(&*cap[0].to_string(), &*result);
        } else if let Some(cap) = on_distance_from_city_re.captures(part) {
            let city_a = cap[1].parse::<City>().unwrap();
            let city_b = cap[2].parse::<City>().unwrap();
            let distance_raw = cap[3].to_string();
            let result = on_distance_from_city(city_a, city_b, distance_raw, &cities, &matrix).to_string();

            condition = condition.replace(&*cap[0].to_string(), &*result);
        } else if let Some(cap) = on_distance_re.captures(part) {
            let city = cap[1].parse::<City>().unwrap();
            let distance_raw = cap[2].to_string();
            let result = on_distance(city, distance_raw, &cities, &matrix).to_string();

            condition = condition.replace(&*cap[0].to_string(), &*result);
        }
        // else if let Some(cap) = on_time_from_city_re.captures(part) {
        //     conditions.push(Condition::OnTimeFromCity(cap[1].to_string(), cap[2].to_string(), cap[3].to_string()));
        // } else if let Some(cap) = on_time_re.captures(part) {
        //     conditions.push(Condition::OnTime(cap[1].to_string(), cap[2].to_string()));
        // }
    }
}

fn parse_rules(rules: Vec<str>) -> Vec<RuleStr> {
    let whitespaces_pattern = Regex::new(r"\s{2,}").unwrap();
    // let pattern = r"([а-яА-Яa-zA-Z-0-9]+)\s+(следует за|по порядку|на дистанции от|на дистанции|на времени от|на времени)(\s+[а-яА-Яa-zA-Z-0-9]+)?\s+([а-яА-Яa-zA-Z-]+|\d+|\[.*?\])";
    let functions = [
        (Regex::new("([а-яА-Яa-zA-Z-]+) следует за ([а-яА-Яa-zA-Z-]+)").unwrap(), "follows(\"$1\", \"$2\")"),
        (Regex::new(r"([а-яА-Яa-zA-Z-]+) по порядку (\d+)").unwrap(), "in_order(\"$1\", $2)"),
        (Regex::new(r"([а-яА-Яa-zA-Z-0-9]+) на дистанции (\d+|\[.*\])").unwrap(), "on_distance(\"$1\", $2)"),
        (Regex::new(r"([а-яА-Яa-zA-Z-0-9]+) на дистанции от ([а-яА-Яa-zA-Z-0-9]+) (\d+|\[.*?\])").unwrap(), "on_distance_city(\"$1\", \"$2\", $3)"),
        (Regex::new(r"([а-яА-Яa-zA-Z-0-9]+) на времени (\d+|\[.*?\])").unwrap(), "on_time(\"$1\", $2)"),
        (Regex::new(r"([а-яА-Яa-zA-Z-0-9]+) на времени от ([а-яА-Яa-zA-Z-0-9]+) (\d+|\[.*?\])").unwrap(), "on_time_city(\"$1\", \"$2\", $3)"),
    ];

    rules.into_iter().map(|rule: str| {
        let mut result = whitespaces_pattern.replace_all(rule, " ").into_owned()
            .replace(" и ", " && ")
            .replace(" или ", " || ");

        for (func_pattern, replacement) in functions {
            result = func_pattern.replace_all(&result, replacement).into_owned()
        }

        result
    })
}