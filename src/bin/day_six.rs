use std::collections::HashMap;
use std::fs;
use std::vec;

fn transitive_closure(mut rules: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    loop {
        let mut rules_added = 0;

        let current_rules = rules.clone();
        for (start, connected_to) in current_rules {
            let connection_count = connected_to.len();

            for current_connection in connected_to.clone() {
                if rules.contains_key(&current_connection) {
                    let mut reachable_via = rules.get(&current_connection).unwrap().clone();
                    reachable_via.retain(|x| *x != start);
                    reachable_via.retain(|x| !rules.get_mut(&start).unwrap().contains(x));
                    rules.get_mut(&start).unwrap().append(&mut reachable_via);
                }
            }

            rules_added = rules_added + (rules.get(&start).unwrap().len() - connection_count);
        }

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
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let (to, from) = (
            line.split(")").nth(0).unwrap(),
            line.split(")").nth(1).unwrap(),
        );

        let rules_vec = rules.entry(from.to_string()).or_insert(vec![]);
        rules_vec.push(to.to_string());
    }

    // Part one - find transitive closure of orbits and output number of connections
    let final_orbit_count = transitive_closure(rules.clone())
        .iter()
        .fold(0, |acc, x| acc + x.1.len());

    println!("Part one: {:?}", final_orbit_count);
}