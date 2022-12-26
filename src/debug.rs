use super::utils::{move_tail, Direction, Record};
use ndarray::{array, Array1};
use std::collections::HashSet;
use std::{error::Error, str::FromStr};

pub fn p9a() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .from_path("data/snake_small.txt")?;
    let iter = rdr.deserialize();
    let mut tail = array![0, 0].view_mut();
    // let mut visited_points: HashSet<&Array1<i32>> = HashSet::new();
    let head = array![0, 0];
    for result in iter {
        let record: Record = result?;
        let direction = Direction::from_str(&record.direction).unwrap();
        let direction_vec = direction.get_direction_vect();
        let mut head = &head + &(record.amount * &direction_vec);
        let tail = move_tail(tail, head, direction_vec);
        println!("tail {}", tail);
        // visited_points.insert(&visited_point);
    }
    // let n_visited_points = visited_points.len();
    // println!("The number of visited points is {}", n_visited_points);
    Ok(())
}
