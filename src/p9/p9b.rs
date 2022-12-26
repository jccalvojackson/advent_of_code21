use super::utils::{move_tail, plot_points, Direction, Record};
use ndarray::{Array, Array1};
use std::collections::HashSet;
use std::{error::Error, str::FromStr};

pub fn p9b() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_path("data/snake.txt")?;
    let iter = rdr.deserialize();
    let number_of_knots = 10;
    let mut knots = Array::zeros((number_of_knots, 2));
    let mut visited_points: HashSet<Array1<i32>> = HashSet::new();
    let mut all_points: HashSet<Array1<i32>> = HashSet::new();
    for result in iter {
        let record: Record = result?;
        let direction = Direction::from_str(&record.direction).unwrap();
        let direction_vec = direction.get_direction_vect();
        for _ in 0..record.amount {
            let mut view_head = knots.row_mut(0);
            view_head += &direction_vec;
            drop(view_head);
            for rowidx in 0..(number_of_knots - 1) {
                move_tail(rowidx, knots.view_mut());
            }
            let tail = knots.row(number_of_knots - 1);
            for rowv in knots.rows() {
                all_points.insert(rowv.to_owned());
            }
            visited_points.insert(tail.to_owned());
        }
    }
    let n_visited_points = visited_points.len();
    println!("The number of visited points is {}", n_visited_points);
    Ok(())
}
