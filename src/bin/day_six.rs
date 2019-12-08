use std::collections::HashMap;
use std::fs;

fn transitive_closure(mut rules: HashMap<String, HashMap<String, i32>>) -> HashMap<String, HashMap<String, i32>> {
    loop {
        let mut rules_added = 0;

        let current_rules = rules.clone();
        for (start, connected_to) in current_rules {
            for (current_connection, dist) in connected_to.clone() {
                if rules.contains_key(&current_connection) {
                    let mut reachable_via = rules.get(&current_connection).unwrap().clone();
                    reachable_via.retain(|x, _| *x != start);

                    for (new_connection, additional_dist) in reachable_via {
                        if !rules.get_mut(&start).unwrap().contains_key(&new_connection) || rules.get_mut(&start).unwrap().get(&new_connection).unwrap() > &(dist + additional_dist) {
                            rules.get_mut(&start).unwrap().insert(new_connection, dist + additional_dist);
                            rules_added += 1;
                        }
                    }
                }
            }
        }

        if rules_added == 0 {
            break;
        }
    }

    rules
}

fn symmetric_closure(mut rules: HashMap<String, HashMap<String, i32>>) -> HashMap<String, HashMap<String, i32>> {
    loop {
        let mut rules_added = 0;

        let current_rules = rules.clone();
        for (start, connected_to) in current_rules {
            for (current_connection, dist) in connected_to.clone() {
                if rules.contains_key(&current_connection) {
                    if !rules.get(&current_connection).unwrap().contains_key(&start)
                        || rules.get(&current_connection).unwrap().get(&start).unwrap() > &dist {
                        rules.get_mut(&current_connection).unwrap().insert(start.to_string(), dist);
                        rules_added += 1;
                    }
                }
            }
        }

        rules = transitive_closure(rules);

        if rules_added == 0 {
            break;
        }
    }

    rules
}

fn main() {
    let input = fs::read_to_string("data/daySix.txt")
        .expect("Unable to read daySix.txt");

    // Start by building a list of direct orbits
    let mut rules: HashMap<String, HashMap<String, i32>> = HashMap::new();

    for line in input.lines() {
        let (to, from) = (
            line.split(")").nth(0).unwrap(),
            line.split(")").nth(1).unwrap(),
        );

        let rules_vec = rules.entry(from.to_string()).or_insert(HashMap::new());
        rules_vec.insert(to.to_string(), 1);
    }

    let rules = transitive_closure(rules);

    // Part one - find transitive closure of orbits and output number of connections
    let final_orbit_count = rules.iter()
        .fold(0, |acc, x| acc + x.1.len());

    println!("Part one: {:?}", final_orbit_count);

    // Part two - breadth-first search to find quickest way to reach Santa
    println!("Part two: {:?}", symmetric_closure(rules).get("YOU").unwrap().get("SAN").unwrap() - 2);
}