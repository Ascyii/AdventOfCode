use std::str::FromStr;

advent_of_code::solution!(5);

#[derive(Debug, PartialEq)]
struct AlmanacMapEntry {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

impl AlmanacMapEntry {
    fn convert(&self, source: u64) -> Option<u64> {
        if self.source_start <= source && source < (self.source_start + self.length) {
            Some(source - self.source_start + self.dest_start)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct AlmanacMap(Vec<AlmanacMapEntry>);

#[derive(Debug, PartialEq)]
struct ValueRange {
    start: u64,
    length: u64,
}

impl AlmanacMap {
    fn convert(&self, source: u64) -> u64 {
        match self
            .0
            .iter()
            .map(|entry| entry.convert(source))
            .find_map(|e| e)
        {
            Some(dest) => dest,
            None => source,
        }
    }

    
}

#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn seed_to_location(&self, seed: u64) -> u64 {
        self.maps.iter().fold(seed, |value, map| map.convert(value))
    }
}

#[derive(Debug, PartialEq)]
struct ParseAlmanacError;

impl FromStr for Almanac {
    type Err = ParseAlmanacError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut seeds = Vec::new();
        let mut maps = Vec::new();

        for (ix, section) in text.split("\n\n").enumerate() {
            if ix == 0 {
                if let Some(seeds_str) = section.strip_prefix("seeds: ") {
                    for seed in seeds_str.split_whitespace().map(u64::from_str) {
                        let seed = seed.map_err(|_| ParseAlmanacError)?;
                        seeds.push(seed);
                    }
                } else {
                    return Err(ParseAlmanacError);
                }
            } else {
                let map: AlmanacMap = section.parse()?;
                maps.push(map);
            }
        }

        Ok(Self { seeds, maps })
    }
}

impl FromStr for AlmanacMap {
    type Err = ParseAlmanacError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut entries = Vec::new();

        for (ix, line) in text.lines().enumerate() {
            if ix == 0 {
                continue;
            }

            let entry: AlmanacMapEntry = line.parse()?;
            entries.push(entry);
        }

        Ok(Self(entries))
    }
}

impl FromStr for AlmanacMapEntry {
    type Err = ParseAlmanacError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut dest_start: Result<u64, Self::Err> = Err(ParseAlmanacError);
        let mut source_start: Result<u64, Self::Err> = Err(ParseAlmanacError);
        let mut length: Result<u64, Self::Err> = Err(ParseAlmanacError);

        for (ix, value) in text
            .split_whitespace()
            .map(|value| u64::from_str(value).map_err(|_| ParseAlmanacError))
            .enumerate()
        {
            match ix {
                0 => dest_start = value,
                1 => source_start = value,
                2 => length = value,
                _ => return Err(ParseAlmanacError),
            }
        }

        let dest_start = dest_start?;
        let source_start = source_start?;
        let length = length?;

        Ok(AlmanacMapEntry {
            dest_start,
            source_start,
            length,
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    if let Ok(almanac) = input.parse::<Almanac>() {
        almanac
            .seeds
            .iter()
            .map(|seed| almanac.seed_to_location(*seed))
            .min()
    } else {
        None
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
