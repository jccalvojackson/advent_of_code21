use super::utils::scenic_score;
extern crate ndarray;
use ndarray::ShapeBuilder;
use ndarray::{Array, Array2};
use std::error::Error;

pub fn p8b() -> Result<(), Box<dyn Error>> {
    let size = 99;
    let mut arr: Array2<u32> = Array::<u32, _>::zeros((size, size).f());
    let tree_rows = include_str!("../../data/trees.txt").lines();
    for (rowidx, tree_row) in tree_rows.enumerate() {
        for (colidx, tree) in tree_row.chars().enumerate() {
            arr[[rowidx, colidx]] = tree.to_digit(10).unwrap();
        }
    }
    let mut max_scenic_score: i32 = 0;
    for rowidx in 0..size {
        for col in 0..size {
            let csc = scenic_score(&arr, rowidx, col, size)?;
            max_scenic_score = if csc > max_scenic_score {
                csc
            } else {
                max_scenic_score
            };
            // println!("mss {} {} {}", max_scenic_score, rowidx, col);
        }
    }
    // let file = File::open("data/trees_small.txt")?;
    // let mut reader = ReaderBuilder::new()
    //     .has_headers(false)
    //     .delimiter('')
    //     .from_reader(file);
    // let array_read: Array2<u64> = reader.deserialize_array2((5, 5))?;
    println!("max scenic score {}", max_scenic_score);
    Ok(())
}
