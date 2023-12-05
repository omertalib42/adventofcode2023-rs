use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn make_reader() -> Result<BufReader<File>, Error> {
    let file = File::open("day5/day5_input.txt")?;
    Ok(BufReader::new(file))
}

fn parse_seeds(reader: &mut BufReader<File>) -> Result<Vec<u64>, Error> {
    let mut line = String::new();
    reader.read_line(&mut line)?;

    Ok(line
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect())
}

fn parse_map(reader: &mut BufReader<File>) -> Result<Vec<(u64, u64, u64)>, Error> {
    let mut map = Vec::new();
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        if line.trim().is_empty() {
            break;
        }
        let values = line
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<u64>>();
        if values.len() == 3 {
            map.push((values[0], values[1], values[2]));
        }
        line.clear();
    }
    Ok(map)
}

fn map_value(value: u64, map: &[(u64, u64, u64)]) -> u64 {
    for &(dest_start, source_start, length) in map {
        if value >= source_start && value < source_start + length {
            return dest_start + (value - source_start);
        }
    }
    value
}

fn main() -> Result<(), Error> {
    let mut reader = make_reader()?;
    let seeds = parse_seeds(&mut reader)?;

    reader.read_line(&mut String::new())?;
    let seed_to_soil = parse_map(&mut reader)?;
    reader.read_line(&mut String::new())?;
    let soil_to_fertilizer = parse_map(&mut reader)?;
    reader.read_line(&mut String::new())?;
    let fertilizer_to_water = parse_map(&mut reader)?;
    reader.read_line(&mut String::new())?;
    let water_to_light = parse_map(&mut reader)?;
    reader.read_line(&mut String::new())?;
    let light_to_temperature = parse_map(&mut reader)?;
    reader.read_line(&mut String::new())?;
    let temperature_to_humidity = parse_map(&mut reader)?;
    reader.read_line(&mut String::new())?;
    let humidity_to_location = parse_map(&mut reader)?;

    let mut lowest_location = u64::MAX;

    for seed in seeds {
        let soil = map_value(seed, &seed_to_soil);
        let fertilizer = map_value(soil, &soil_to_fertilizer);
        let water = map_value(fertilizer, &fertilizer_to_water);
        let light = map_value(water, &water_to_light);
        let temperature = map_value(light, &light_to_temperature);
        let humidity = map_value(temperature, &temperature_to_humidity);
        let location = map_value(humidity, &humidity_to_location);

        lowest_location = lowest_location.min(location);
    }

    println!("Lowest location: {}", lowest_location);
    Ok(())
}
