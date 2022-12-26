// use super::utils::{move_tail, Direction, Record};
// use ndarray::{array, Array1};
// use std::collections::HashSet;
// use std::{error::Error, str::FromStr};

// pub fn p9a() -> Result<(), Box<dyn Error>> {
//     let mut rdr = csv::ReaderBuilder::new()
//         .has_headers(false)
//         .delimiter(b' ')
//         .from_path("data/snake.txt")?;
//     let iter = rdr.deserialize();
//     let mut tail = array![0, 0];
//     let mut visited_points: HashSet<Array1<i32>> = HashSet::new();
//     let mut head = array![0, 0];
//     for result in iter {
//         let mut view_head = head.view_mut();
//         let view_tail = tail.view_mut();
//         let record: Record = result?;
//         let direction = Direction::from_str(&record.direction).unwrap();
//         let direction_vec = direction.get_direction_vect();
//         view_head += &(record.amount * &direction_vec);
//         let nsteps = move_tail(view_tail, view_head, direction_vec.clone());
//         visited_points.insert(tail.clone());
//         for _ in 0..nsteps {
//             tail += &direction_vec;
//             visited_points.insert(tail.clone());
//         }
//     }
//     let n_visited_points = visited_points.len();
//     println!("The number of visited points is {}", n_visited_points);
//     Ok(())
// }
