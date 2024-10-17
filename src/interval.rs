use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Interval {
    pub start: i64,
    pub range: i64,
}

impl Interval {
    pub fn new(start: i64, range: i64) -> Self {
        Interval { start, range }
    }

    pub fn end(&self) -> i64 {
        self.start + self.range
    }

    pub fn has_intersection(&self, i: &Interval) -> bool {
        let start_in_range = self.start >= i.start && self.start < i.end();
        let end_in_range = self.end() > i.start && self.end() <= i.end();

        start_in_range || end_in_range
    }

    pub fn is_contained_in(&self, i: &Interval) -> bool {
        let start_contained = self.start >= i.start && self.start < i.start + i.range;
        let end_contained = self.end() > i.start && self.end() <= i.end();

        start_contained && end_contained
    }

    /*
        self -> intervalo original
        other -> intervalo do mapper
    */
    pub fn subtract(&self, other: &Interval) -> Vec<Interval> {
        let mut result = Vec::new();

        if !self.has_intersection(other) && !other.has_intersection(self) {
            result.push(self.clone());
            return result;
        }

        let self_end = self.start + self.range;
        let other_end = other.start + other.range;

        let self_end_is_included = self_end < other_end && self_end > other.start;
        let other_end_is_included = other_end < self_end && other_end > self.start;

        if self.start < other.start {
            // println!("A1");
            let start = self.start;
            let range = if self_end < other.start {
                self.range
            } else {
                other.start - self.start
            };
            result.push(Interval::new(start, range));
        } else {
            // println!("A2");
            let start = self.start;
            let range = if other_end_is_included {
                other_end - self.start
            } else {
                self.range
            };
            result.push(Interval::new(start, range));
        }

        if self_end > other_end && self.start < other_end {
            // println!("B1");
            let start = other_end;
            let range = self_end - other_end;

            result.push(Interval::new(start, range));
        } else if self_end_is_included {
            // println!("B2");
            let start = other.start;
            let range = self_end - other.start;
            result.push(Interval::new(start, range));
        }

        if other.is_contained_in(&self) {
            // println!("C1");
            let start = other.start;
            let range = other.range;
            result.push(Interval::new(start, range));
        }

        result
    }
}

impl FromStr for Interval {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let start = parts
            .next()
            .ok_or("No start")?
            .parse::<i64>()
            .map_err(|_| "Start is not a number")?;

        let range = parts
            .next()
            .ok_or("No range")?
            .parse::<i64>()
            .map_err(|_| "Range is not a number")?;

        Ok(Interval::new(start, range))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_contained() {
        let i1 = Interval::new(0, 5);
        let i2 = Interval::new(0, 4);
        let i3 = Interval::new(1, 4);
        let i4 = Interval::new(1, 5);
        let i5 = Interval::new(2, 2);

        assert_eq!(i2.is_contained_in(&i1), true);
        assert_eq!(i3.is_contained_in(&i1), true);
        assert_eq!(i4.is_contained_in(&i1), false);
        assert_eq!(i5.is_contained_in(&i1), true);
        assert_eq!(i1.is_contained_in(&i5), false);
    }

    #[test]
    fn subtract_1() {
        /*
        self    -> |--------------|
        other   ->    |-----|
        case1   -> |--|-----|-----|
        */
        let i1 = Interval::new(50, 10);
        let i2 = Interval::new(55, 3);

        let r1 = Interval::new(50, 5);
        let r2 = Interval::new(55, 3);
        let r3 = Interval::new(58, 2);

        let result = i1.subtract(&i2);
        assert_eq!(result.len(), 3);
        assert_eq!(result.contains(&r1), true);
        assert_eq!(result.contains(&r2), true);
        assert_eq!(result.contains(&r3), true);
    }

    #[test]
    fn subtract_2() {
        /*
        self    -> |--------------|
        other   ->    |--------------|
        case2   -> |--|-----------|
        */
        let i1 = Interval::new(50, 10);
        let i2 = Interval::new(55, 10);

        let r1 = Interval::new(50, 5);
        let r2 = Interval::new(55, 5);

        let result = i1.subtract(&i2);
        assert_eq!(result.len(), 2);
        assert_eq!(result.contains(&r1), true);
        assert_eq!(result.contains(&r2), true);
    }

    #[test]
    fn subtract_3() {
        /*
        self    ->    |------------|
        other   -> |-----|
        case3   ->    |--|---------|
        */

        let i1 = Interval::new(55, 10);
        let i2 = Interval::new(50, 8);

        let r1 = Interval::new(55, 3);
        let r2 = Interval::new(58, 7);

        let result = i1.subtract(&i2);
        println!("{:?}", result);
        assert_eq!(result.len(), 2);
        assert_eq!(result.contains(&r1), true);
        assert_eq!(result.contains(&r2), true);
    }

    #[test]
    fn subtract_4() {
        /*
        self    ->    |-----|
        other   ->          |-------|
        case4   ->    |-----|
        */

        let i1 = Interval::new(50, 5);
        let i2 = Interval::new(55, 10);

        let r1 = Interval::new(50, 5);

        let result = i1.subtract(&i2);
        assert_eq!(result.len(), 1);
        assert_eq!(result.contains(&r1), true);
    }

    #[test]
    fn subtract_5() {
        /*
        self    ->        |-----|
        other   ->    |---|
        case5   ->        |-----|
        */

        let i1 = Interval::new(55, 5);
        let i2 = Interval::new(50, 5);

        let r1 = Interval::new(55, 5);

        let result = i1.subtract(&i2);
        println!("{:?}", result);
        assert_eq!(result.len(), 1);
        assert_eq!(result.contains(&r1), true);
    }
}
