use std::cmp::max;
use std::path::PathBuf;
use std::collections::{HashSet, HashMap};

use crate::utils::read_lines;


fn parse_valve_map(valve_map: &String) -> (String, u64, HashSet<String>) {
    let flow_rate_tunnels: Vec<&str> = valve_map.split("; ").collect();
    let flow_rate_str: Vec<&str> = flow_rate_tunnels[0].split(" ").collect();
    let valve_key: String = flow_rate_str[1].to_string();
    let flow_rate = flow_rate_str[4].split("=").nth(1).unwrap().parse::<u64>().unwrap();

    let split_word: String;
    if flow_rate_tunnels[1].contains("valves") { split_word = "valves ".to_string()} else { split_word = "valve ".to_string() }
    let tunnels: HashSet<String> = flow_rate_tunnels[1].split(&split_word).nth(1).unwrap().split(", ").map(|x| x.to_string()).collect();

    return (valve_key, flow_rate, tunnels)
}


fn distance(src: String,
            dst: String,
            tunnel_system: &HashMap<String, HashSet<String>>) -> u64 {
    assert!(tunnel_system.contains_key(&src));
    assert!(tunnel_system.contains_key(&dst));

    let mut next_nodes: Vec<String> = vec![src];
    let mut cache: Vec<String> = vec![];

    let mut ret: u64 = 0;

    loop {
        while let Some(node) = next_nodes.pop() {
            if node == dst { return ret }
            for nnode in tunnel_system.get(&node).unwrap().iter() { cache.push(nnode.to_string()); }
        }
        while let Some(node) = cache.pop() { next_nodes.push(node); }
        ret += 1;
    }
}


fn find_max_flow(time: u64,
                 room: String,
                 flow: u64,
                 activated: &mut HashSet<String>,
                 valves: &HashMap<String, u64>,
                 tunnel_system: &HashMap<String, HashSet<String>>,
                 distance_cache: &HashMap<(String, String), u64>) -> u64 {
    if time == 0 { return flow }

    let mut to_activate: Vec<(u64, String)> = valves.iter()
        .filter(|(k, v)|
            // flow must be more than 0.
            **v != 0 &&
            // has not been activated
            !activated.contains(*k) &&
            // there is enough time to reach the valve and activate it
            1 + distance_cache.get(&(k.to_string(), room.to_string())).unwrap() <= time)
        .map(|(k, v)|
            (
                v*(time - 1 - distance_cache.get(&(k.to_string(), room.to_string())).unwrap()),
                k.to_string()
            )
        )
        .collect();

    to_activate.sort();
    to_activate.reverse();

    let mut all_flows: Vec<u64> = vec![];

    for (_, next_room) in to_activate.iter() {
        let dist = distance_cache.get(&(room.to_string(), next_room.to_string())).unwrap();
        activated.insert(next_room.to_string());

        all_flows.push(
            find_max_flow(
                time-dist-1,
                next_room.to_string(),
                flow + (time-dist-1) * valves.get(next_room).unwrap(),
                activated,
                valves,
                tunnel_system,
                distance_cache,
            )
        );

        activated.remove(next_room);
    }

    return *all_flows.iter().max().unwrap_or(&flow)
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input16.txt".to_string()
    ].iter().collect();

    let mut valves: HashMap<String, u64> = HashMap::new();
    let mut tunnel_system: HashMap<String, HashSet<String>> = HashMap::new();

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(valve_map) = line {
                let key_fr_tunnels = parse_valve_map(&valve_map);
                valves.insert(key_fr_tunnels.0.to_string(), key_fr_tunnels.1);
                tunnel_system.insert(key_fr_tunnels.0.to_string(), key_fr_tunnels.2);
            }
        }
    }

    let mut distance_cache: HashMap<(String, String), u64> = HashMap::new();
    let good_nodes: Vec<String> = valves.iter().filter(|(_, v)| **v > 0).map(|(k, _)| k.to_string()).collect();
    for idx in 0..good_nodes.len() {
        let src = good_nodes[idx].to_string();
        for jdx in idx..good_nodes.len() {
            let dst = good_nodes[jdx].to_string();
            let dist = distance(src.to_string(), dst.to_string(), &tunnel_system);
            distance_cache.insert((src.to_string(), dst.to_string()), dist);
            distance_cache.insert((dst.to_string(), src.to_string()), dist);
        }

        let dst = "AA".to_string();
        let dist = distance(src.to_string(), dst.to_string(), &tunnel_system);
        distance_cache.insert((src.to_string(), dst.to_string()), dist);
        distance_cache.insert((dst.to_string(), src.to_string()), dist);

    }

    let mut activated: HashSet<String> = HashSet::new();

    let max_flow: u64 = find_max_flow(
        30,
        "AA".to_string(),
        0,
        &mut activated,
        &valves,
        &tunnel_system,
        &distance_cache,
    );

    let mut max_flow_he: u64 = 0;

    for idx in 0..(2_i32.pow((good_nodes.len() - 1) as u32) - 1) as u64 {
        let mut activated_h: HashSet<String> = HashSet::new();
        let mut activated_e: HashSet<String> = HashSet::new();
        let mut val = idx;

        for node in good_nodes.iter() {
            if val % 2 == 0 {
                activated_h.insert(node.to_string());
            } else {
                activated_e.insert(node.to_string());
            }
            val /= 2;
        }

        if activated_e.len() - activated_h.len() < 2 {
            let flow_h = find_max_flow(
                26,
                "AA".to_string(),
                0,
                &mut activated_h,
                &valves,
                &tunnel_system,
                &distance_cache,
            );

            let flow_e = find_max_flow(
                26,
                "AA".to_string(),
                0,
                &mut activated_e,
                &valves,
                &tunnel_system,
                &distance_cache,
            );

            max_flow_he = max(max_flow_he, flow_h + flow_e);
        }
    }

    return (15, max_flow, max_flow_he)
}
