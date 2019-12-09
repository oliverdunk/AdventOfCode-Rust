use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn transitive_closure(rules: &mut HashMap<String, HashSet<String>>) {
    loop {
        let mut rules_added = 0;

        let current_rules = rules.clone();
        for (start, connected_to) in current_rules {
            let connection_count = connected_to.len();

            for current_connection in connected_to {
                let reachable_via = rules.get(&current_connection);

                if reachable_via.is_some() {
                    let mut reachable_via = reachable_via.unwrap().clone();
                    reachable_via.retain(|x| *x != start);
                    rules.get_mut(&start).unwrap().extend(reachable_via);
                }
            }

            rules_added = rules_added + (rules.get(&start).unwrap().len() - connection_count);
        }
        
        if rules_added == 0 {
            break;
        }
    }
}

fn part_two(mut rules: HashMap<String, HashSet<String>>) -> i32 {
    let mut steps = 0;
    
    let mut reached = HashSet::new();
    reached.insert("YOU".to_string());
    
    loop {
        let mut to_add = HashSet::new();
        
        for start in &reached {
            // Look at what orbits this starting position
            to_add.extend(rules.entry(start.to_string()).or_default().clone());
            
            // Look at what this starting position orbits
            for (maybe_reachable, reachable_from) in &rules {
                if reachable_from.contains(start) {
                    to_add.insert(maybe_reachable.to_string());
                }
            }
        }
        
        steps = steps + 1;
        
        if to_add.contains(&"SAN".to_string()) {
            return steps - 2;
        }
        
        reached.extend(to_add);
    }
}

fn main() {
    let input = fs::read_to_string("data/daySix.txt")
        .expect("Unable to read daySix.txt");

    // Start by building a list of direct orbits
    let mut rules: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let (to, from) = (
            line.split(")").nth(0).unwrap(),
            line.split(")").nth(1).unwrap(),
        );

        let rules_vec = rules.entry(from.to_string()).or_insert(HashSet::new());
        rules_vec.insert(to.to_string());
    }

    // Part one - find transitive closure of orbits and output number of connections
    let mut rules_closure = rules.clone();
    transitive_closure(&mut rules_closure);

    let final_orbit_count = rules_closure
        .iter()
        .fold(0, |acc, x| acc + x.1.len());

    println!("Part one: {:?}", final_orbit_count);
    println!("Part two: {:?}", part_two(rules));
}