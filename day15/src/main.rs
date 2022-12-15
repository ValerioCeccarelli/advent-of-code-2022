use std::fs;

struct Sensor {
    x: i32,
    y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

fn main() {
    let path = "input/input.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let sensors = parse_file(&contents);

    let result1 = solution_part1(&sensors);
    let result2 = solution_part2(&sensors);

    println!("Solution part 1: {}", result1);
    println!("Solution part 2: {}", result2);
}

fn solution_part2(sensors: &Vec<Sensor>) -> i128 {
    let mut result = 0;
    for i in 0..4000000 {
        let ranges = find_ranges(&sensors, i);
        let res = group_ranges(&ranges);
        if res.len() != 1 {
            println!("{:?} {}", res, i);
            let x = (res[0].1 + 1) as i128;
            result = x * 4000000 + (i as i128);
            break;
        }
    }

    result
}

fn solution_part1(sensors: &Vec<Sensor>) -> i32 {
    let ranges = find_ranges(&sensors, 2000000);
    let res = group_ranges(&ranges);
    res[0].1 - res[0].0
}

fn group_ranges(sensors: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut sensors = sensors.clone();
    sensors.sort_by_key(|a| a.0);

    let mut ranges = Vec::new();

    for (min_x, max_x) in sensors {
        let mut found = false;
        for (min, max) in &mut ranges {
            if min_x >= *min && min_x <= *max + 1 {
                *min = i32::min(*min, min_x);
                *max = i32::max(*max, max_x);
                found = true;
                break;
            }
            if max_x >= *min + 1 && max_x <= *max {
                *min = i32::min(*min, min_x);
                *max = i32::max(*max, max_x);
                found = true;
                break;
            }
        }

        if !found {
            ranges.push((min_x, max_x));
        }
    }

    ranges
}

fn find_ranges(sensors: &Vec<Sensor>, line_y: i32) -> Vec<(i32, i32)> {
    let mut ranges = Vec::new();

    for sensor in sensors {
        let distance = i32::abs(sensor.x - sensor.beacon_x) + i32::abs(sensor.y - sensor.beacon_y);

        let diff = i32::abs(sensor.y - line_y);
        let offset = distance - diff;

        if offset >= 0 {
            let min_x = sensor.x - offset;
            let max_x = sensor.x + offset;
            ranges.push((min_x, max_x));
        }
    }

    ranges
}

fn parse_file(contents: &String) -> Vec<Sensor> {
    let mut sensors = Vec::new();

    for line in contents.lines() {
        let mut line = line.split(':');
        let mut sensors_str = line.next().unwrap()[10..].split(", ");
        let mut beacons_str = line.next().unwrap()[22..].split(", ");

        let sensor_x = sensors_str.next().unwrap()[2..].parse::<i32>().unwrap();
        let sensor_y = sensors_str.next().unwrap()[2..].parse::<i32>().unwrap();

        let beacon_x = beacons_str.next().unwrap()[2..].parse::<i32>().unwrap();
        let beacon_y = beacons_str.next().unwrap()[2..].parse::<i32>().unwrap();

        sensors.push(Sensor {
            x: sensor_x,
            y: sensor_y,
            beacon_x: beacon_x,
            beacon_y: beacon_y,
        });
    }

    sensors
}
