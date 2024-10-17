use std::str::FromStr;

use crate::interval::Interval;

#[derive(Debug, PartialEq)]
pub struct Mapper {
    pub source: String,
    pub destination: String,
    pub movies: Vec<(Interval, Interval)>,
}

impl Mapper {
    pub fn new(source: String, destination: String) -> Self {
        Mapper {
            source,
            destination,
            movies: vec![],
        }
    }

    pub fn add_movie(&mut self, source: Interval, destination: Interval) {
        self.movies.push((source, destination));
    }
}

impl FromStr for Mapper {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let from_to = lines.next().unwrap().to_string();
        let binding = from_to.replace(" map:", "");
        let mut parts = binding.split("-to-");
        let from = parts.next().unwrap().to_string();
        let to = parts.next().unwrap().to_string();

        let mut mapper = Mapper::new(from, to);

        for line in lines {
            if line.is_empty() {
                break;
            }

            let mut parts = line.split_whitespace();
            let destination: i64 = parts.next().unwrap().parse().unwrap();
            let source: i64 = parts.next().unwrap().parse().unwrap();
            let range: i64 = parts.next().unwrap().parse().unwrap();

            let destination = Interval::new(destination, range);
            let source = Interval::new(source, range);

            mapper.add_movie(source, destination);
        }
        Ok(mapper)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapper_from_str() {
        let input = "seed-to-soil map:\n50 98 2\n52 50 48";
        let mapper = Mapper::from_str(input).unwrap();
        assert_eq!(mapper.source, "seed".to_string());
        assert_eq!(mapper.destination, "soil".to_string());

        let (source, destination) = &mapper.movies[0];
        assert_eq!(source.start, 98);
        assert_eq!(source.range, 2);
        assert_eq!(destination.start, 50);
        assert_eq!(destination.range, 2);

        let (source, destination) = &mapper.movies[1];
        assert_eq!(source.start, 50);
        assert_eq!(source.range, 48);
        assert_eq!(destination.start, 52);
        assert_eq!(destination.range, 48);
    }
}
