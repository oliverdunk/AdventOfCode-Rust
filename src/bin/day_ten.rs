use std::fs;

fn atan2(from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> f32 {
    (to_y as f32 - from_y as f32).atan2(to_x as f32 - from_x as f32)
}

fn dist(from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> f32 {
    f32::sqrt((i32::pow(to_x - from_x, 2) + i32::pow(to_y - from_y, 2)) as f32)
}

fn can_reach(grid: &Vec<Vec<char>>, from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> bool {
    let angle_to_target = atan2(from_x, from_y, to_x, to_y);
    let dist_to_target = dist(from_x, from_y, to_x, to_y);

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != '#' {
                continue
            }

            if x == from_x as usize && y == from_y as usize {
                continue
            }

            if x == to_x as usize && y == to_y as usize {
                continue
            }

            let angle_to_asteroid = atan2(from_x, from_y, x as i32, y as i32);
            let dist_to_asteroid = dist(from_x, from_y, x as i32, y as i32);

            if angle_to_target == angle_to_asteroid && dist_to_asteroid < dist_to_target {
                return false;
            }

        }
    }

    true
}

fn reachable_asteroids(grid: &Vec<Vec<char>>, from_x: i32, from_y: i32) -> i32 {
    let mut reachable = 0;

    if *grid.get(from_y as usize).unwrap().get(from_x as usize).unwrap() != '#' {
        return 0;
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if x as i32 == from_x && y as i32 == from_y {
                continue
            }

            if *cell != '#' {
                continue
            }

            if can_reach(grid, from_x, from_y, x as i32, y as i32) {
                reachable = reachable + 1;
            }
        }
    }

    reachable
}

fn main() {
    let input = fs::read_to_string("data/dayTen.txt")
        .expect("Unable to read dayTen.txt");

    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let current_grid = grid.clone();

    let max_visible: i32 = grid.iter()
        .enumerate()
        .map(|(y, row)| {
            let grid_ref = &current_grid;
            row.iter()
                .enumerate()
                .map(move |(x, _cell)| reachable_asteroids(grid_ref, x as i32, y as i32))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("Part one: {}", max_visible);
}