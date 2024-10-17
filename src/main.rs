use std::fs;
use std::str::FromStr;

mod interval;
mod mapper;

fn main() {
    let str = fs::read_to_string("input.txt").unwrap();
    let mut lines = str.split("\n\n");

    let first_line = lines.next().unwrap();
    let first_line = first_line.replace("seeds: ", "");
    let numbers = first_line
        .split_whitespace()
        .map(|s| i64::from_str(s).unwrap())
        .collect::<Vec<i64>>();

    let seeds = numbers
        .chunks(2)
        .map(|w| interval::Interval::new(w[0], w[1]))
        .collect::<Vec<interval::Interval>>();

    let mut mappers = Vec::new();

    while let Some(line) = lines.next() {
        let mapper = mapper::Mapper::from_str(&line).unwrap();
        mappers.push(mapper);
    }

    let mut next_mapper: Option<String> = Some("seed".to_string());

    let mut result = seeds.clone();
    while next_mapper.is_some() {
        let mapper = mappers
            .iter()
            .find(|m| m.source == *next_mapper.as_ref().unwrap());

        if mapper.is_none() {
            break;
        }

        let mapper = mapper.unwrap();

        next_mapper = Some(mapper.destination.clone());

        let mut values_with_intersection = result
            .iter()
            .filter(|i| mapper.movies.iter().any(|m| i.has_intersection(&m.0)))
            .map(|i| i.clone())
            .collect::<Vec<interval::Interval>>();

        let mut values_without_intersection = result
            .iter()
            .filter(|i| !values_with_intersection.contains(i))
            .map(|i| i.clone())
            .collect::<Vec<interval::Interval>>();

        while !values_with_intersection.is_empty() {
            let value = values_with_intersection.pop().unwrap();

            let interval_with_intersection =
                mapper.movies.iter().find(|m| value.has_intersection(&m.0));

            if interval_with_intersection.is_none() {
                values_without_intersection.push(value);
                continue;
            }

            let interval_with_intersection = interval_with_intersection.unwrap();

            if value.is_contained_in(&interval_with_intersection.0) {
                let offset =
                    interval_with_intersection.1.start - interval_with_intersection.0.start;
                let new_start = value.start + offset;
                let new_range = value.range;
                let new_interval = interval::Interval::new(new_start, new_range);
                values_without_intersection.push(new_interval);
            } else {
                let mut new_intervals = value.subtract(&interval_with_intersection.0);
                values_with_intersection.append(&mut new_intervals);
            }
        }

        result = values_without_intersection.clone();
    }
    println!("min: {:?}", result.iter().min_by_key(|i| i.start));
}
