use gnuplot::{AxesCommon, Caption, Color, Figure, Fix};
use ndarray::{array, Array1, Array2, ArrayView1, ArrayViewMut1, ArrayViewMut2};
use serde::Deserialize;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl Direction {
    pub fn get_direction_vect(self) -> Array1<i32> {
        match self {
            Direction::Up => array![0, 1],
            Direction::Down => array![0, -1],
            Direction::Left => array![-1, 0],
            Direction::Right => array![1, 0],
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub direction: String,
    pub amount: i32,
}

pub fn move_tail(rowidx: usize, mut knots: ArrayViewMut2<i32>) {
    let head = knots.row(rowidx);
    let tail = knots.row(rowidx + 1);
    let delta = &head - &tail;
    if touching(delta.clone()) {
        return;
    }
    let dir_vect = delta.mapv(|el| if el == 0 { 0 } else { el / el.abs() });
    let mut tail = knots.row_mut(rowidx + 1);
    tail += &dir_vect;
}

fn touching(delta: Array1<i32>) -> bool {
    let distance_2 = delta.dot(&delta);
    distance_2 < 4
}

pub fn plot_points(points: &Array2<i32>) {
    let mut fg = Figure::new();
    let ax = fg.axes2d();
    ax.set_title("Scatter Plot of Points", &[]);
    ax.set_x_label("X-coordinate", &[]);
    ax.set_y_label("Y-coordinate", &[]);
    ax.set_x_range(Fix(-14.), Fix(14.));
    ax.set_y_range(Fix(-14.), Fix(14.));
    ax.points(
        points.rows().into_iter().map(|vec| vec[0]),
        points.rows().into_iter().map(|vec| vec[1]),
        &[Caption("Points"), Color("red")],
    );
    fg.show().unwrap();
}

// pub fn plot_visited_points(visited_points: &HashSet<Array1<i32>>) {
//     let n_visited_points = visited_points.len();
//     let mut points: [(i32, i32); 34] = [(0, 0); 34];
//     let mut idx = 0;
//     for pt in visited_points {
//         points[idx] = (pt[0], pt[1]);
//         idx += 1
//     }
//     plot_points(&points);
// }
