use std::cmp::max;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

use fancy_regex::Regex;

use crate::utils::read_lines;


type RobotArray = [u64; 4];


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Resources {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,
}


#[derive(Debug, Clone)]
struct BluePrint {
    ore_robot: u64,
    clay_robot: u64,
    obsidian_robot: (u64, u64),
    geode_robot: (u64, u64),
}


#[derive(Debug)]
enum RobotType {
    ORE,
    CLAY,
    OBSIDIAN,
    GEODE,
}


type CacheKey = (u64, Resources, RobotArray);


impl fmt::Display for RobotType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RobotType::ORE => write!(f, "ore"),
            RobotType::CLAY => write!(f, "clay"),
            RobotType::OBSIDIAN => write!(f, "obsidian"),
            RobotType::GEODE => write!(f, "geode"),
        }
    }
}


const ALL_ROBOTS: [RobotType; 4] = [RobotType::ORE, RobotType::CLAY, RobotType::OBSIDIAN, RobotType::GEODE];


fn extract_value_from_regex(re: Regex, word: &String) -> u64 {
    match re.find(word) {
        Ok(Some(result)) => { result.as_str().parse::<u64>().unwrap() },
        _ => panic!(),
    }
}


fn parse_bp(bp_str: &String) -> BluePrint {
    let mut bp: BluePrint = BluePrint { ore_robot: 0, clay_robot: 0, obsidian_robot: (0, 0), geode_robot: (0, 0) };

    let re_clay: Regex = Regex::new(&format!(r"(?<=and )\d+(?= clay.)")).unwrap();
    let re_obsidian: Regex = Regex::new(&format!(r"(?<=and )\d+(?= obsidian.)")).unwrap();

    let clay_amnt: u64 = extract_value_from_regex(re_clay, bp_str);
    let obsidian_amnt: u64 = extract_value_from_regex(re_obsidian, bp_str);

    let mut ore_amnt: u64;
    for robot in ALL_ROBOTS {

        let re_ore: Regex = Regex::new(&format!(r"(?<=Each {} robot costs )\d+(?= ore.)", robot.to_string())).unwrap();
        ore_amnt = extract_value_from_regex(re_ore, bp_str);

        match robot {
            RobotType::ORE => { bp.ore_robot = ore_amnt; },
            RobotType::CLAY => { bp.clay_robot = ore_amnt; },
            RobotType::OBSIDIAN => { bp.obsidian_robot = (ore_amnt, clay_amnt); },
            RobotType::GEODE => { bp.geode_robot = (ore_amnt, obsidian_amnt); },
        }
    }

    return bp
}


fn ceil(a: u64, b: u64) -> u64 {
    return (a / b) + (if a % b == 0 { 0 } else { 1 })
}


fn update_resources(resources: &mut Resources, robots: &RobotArray, delta: u64) {
    resources.ore += delta * robots[0];
    resources.clay += delta * robots[1];
    resources.obsidian += delta * robots[2];
    resources.geode += delta * robots[3];
}


fn run_blueprint_step(bp: &BluePrint,
                      time: u64,
                      resources: &Resources,
                      robots: &RobotArray,
                      max_ore: u64,
                      max_clay: u64,
                      max_obsidian: u64,
                      cache: &mut HashMap<CacheKey, u64>) -> u64 {

    let key: CacheKey = (time, resources.clone(), robots.clone());
    if cache.contains_key(&key) { return *cache.get(&key).unwrap() }

    if time == 0 { return resources.geode }

    let mut geodes: Vec<u64> = vec![];
    let mut time_delta: u64;

    // try to make geode robot
    if robots[0] > 0 && robots[2] > 0 {
        if resources.ore >= bp.geode_robot.0 && resources.obsidian >= bp.geode_robot.1 {
            time_delta = 1;
        } else if resources.ore >= bp.geode_robot.0 {
            time_delta = 1 + ceil(bp.geode_robot.1 - resources.obsidian, robots[2]);
        } else if resources.obsidian >= bp.geode_robot.1 {
            time_delta = 1 + ceil(bp.geode_robot.0 - resources.ore, robots[0]);
        } else {
            time_delta = 1 + max(ceil(bp.geode_robot.1 - resources.obsidian, robots[2]),
                                 ceil(bp.geode_robot.0 - resources.ore, robots[0]));
        }
        if time_delta <= time {
            let mut new_resources: Resources = resources.clone();
            update_resources(&mut new_resources, robots, time_delta);
            new_resources.ore -= bp.geode_robot.0;
            new_resources.obsidian -= bp.geode_robot.1;

            let new_robots: RobotArray = robots.clone();
            new_resources.geode += time-time_delta;
            geodes.push(run_blueprint_step(bp, time-time_delta, &new_resources, &new_robots, max_ore, max_clay, max_obsidian, cache));
        }
    }
    // try to make obsidian robot
    if robots[0] > 0 && robots[1] > 0 && robots[2] < max_obsidian {
        if resources.ore >= bp.obsidian_robot.0 && resources.clay >= bp.obsidian_robot.1 {
            time_delta = 1;
        } else if resources.ore >= bp.obsidian_robot.0 {
            time_delta = 1 + ceil(bp.obsidian_robot.1 - resources.clay, robots[1]);
        } else if resources.clay >= bp.obsidian_robot.1 {
            time_delta = 1 + ceil(bp.obsidian_robot.0 - resources.ore, robots[0]);
        } else {
            time_delta = 1 + max(ceil(bp.obsidian_robot.1 - resources.clay, robots[1]),
                                 ceil(bp.obsidian_robot.0 - resources.ore, robots[0]));
        }
        if time_delta <= time {
            let mut new_resources: Resources = resources.clone();
            update_resources(&mut new_resources, robots, time_delta);
            new_resources.ore -= bp.obsidian_robot.0;
            new_resources.clay -= bp.obsidian_robot.1;

            let mut new_robots: RobotArray = robots.clone();
            new_robots[2] += 1;
            geodes.push(run_blueprint_step(bp, time-time_delta, &new_resources, &new_robots, max_ore, max_clay, max_obsidian, cache));
        }
    }
    // try to make clay robot
    if robots[0] > 0 && robots[1] < max_clay {
        if resources.ore >= bp.clay_robot { time_delta = 1; }
        else { time_delta = 1 + ceil(bp.clay_robot - resources.ore, robots[0]); }

        if time_delta <= time {
            let mut new_resources: Resources = resources.clone();
            update_resources(&mut new_resources, robots, time_delta);
            new_resources.ore -= bp.clay_robot;

            let mut new_robots: RobotArray = robots.clone();
            new_robots[1] += 1;
            geodes.push(run_blueprint_step(bp, time-time_delta, &new_resources, &new_robots, max_ore, max_clay, max_obsidian, cache));
        }
    }
    // try to make ore robot
    if robots[0] > 0 && robots[0] < max_ore {
        if resources.ore >= bp.ore_robot { time_delta = 1; }
        else { time_delta = 1 + ceil(bp.ore_robot - resources.ore, robots[0]); }

        if time_delta <= time {
            let mut new_resources: Resources = resources.clone();
            update_resources(&mut new_resources, robots, time_delta);
            new_resources.ore -= bp.ore_robot;

            let mut new_robots: RobotArray = robots.clone();
            new_robots[0] += 1;
            geodes.push(run_blueprint_step(bp, time-time_delta, &new_resources, &new_robots, max_ore, max_clay, max_obsidian, cache));
        }
    }

    let result: u64 = *geodes.iter().max().unwrap_or(&(time * robots[3] + resources.geode));
    cache.insert(key, result);

    return result
}


fn find_top3_prod(top3: Vec<BluePrint>) -> u64 {
    let mut ret: u64 = 1;
    let mut max_ore: u64;
    let mut max_clay: u64;
    let mut max_obsidian: u64;

    for bp in top3 {
        max_ore = *[bp.ore_robot, bp.clay_robot, bp.obsidian_robot.0, bp.geode_robot.0].iter().max().unwrap();
        max_clay = bp.obsidian_robot.1;
        max_obsidian = bp.geode_robot.1;

        let resources: Resources = Resources { ore: 0, clay: 0, obsidian: 0, geode: 0 };
        let robots: RobotArray = [1, 0, 0, 0];
        let mut cache: HashMap<CacheKey, u64> = HashMap::new();
        ret *= run_blueprint_step(&bp, 32, &resources, &robots, max_ore, max_clay, max_obsidian, &mut cache);
    }
    return ret
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input19.txt".to_string()
    ].iter().collect();

    let mut max_geodes: Vec<u64> = vec![];

    let mut max_ore: u64;
    let mut max_clay: u64;
    let mut max_obsidian: u64;

    let mut bp: BluePrint;
    let mut top3: Vec<BluePrint> = vec![];
    if let Ok(lines) = read_lines(data_path) {
        for (idx, line) in lines.enumerate() {
            if let Ok(bp_str) = line {
                bp = parse_bp(&bp_str);

                if idx < 3 { top3.push(bp.clone()); }

                max_ore = *[bp.ore_robot, bp.clay_robot, bp.obsidian_robot.0, bp.geode_robot.0].iter().max().unwrap();
                max_clay = bp.obsidian_robot.1;
                max_obsidian = bp.geode_robot.1;

                let resources: Resources = Resources { ore: 0, clay: 0, obsidian: 0, geode: 0 };
                let robots: RobotArray = [1,0,0,0];
                let mut cache: HashMap<CacheKey, u64> = HashMap::new();
                max_geodes.push(run_blueprint_step(&bp, 24, &resources, &robots, max_ore, max_clay, max_obsidian, &mut cache));
            }
        }
    }

    return (18, max_geodes.iter().zip(1..(1+max_geodes.len())).map(|(x, y)| x * y as u64).sum(), find_top3_prod(top3))
}
