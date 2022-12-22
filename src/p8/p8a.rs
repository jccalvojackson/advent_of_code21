use ndarray_stats::QuantileExt;
use std::error::Error;
extern crate ndarray;
extern crate scan_fmt;
use ndarray::ShapeBuilder;
use ndarray::{s, Array, Array2};

fn is_visible(arr: &Array2<u32>, rowidx: usize, colidx: usize) -> Result<bool, Box<dyn Error>> {
    if colidx == 0 || colidx == 98 || rowidx == 0 || rowidx == 98 {
        return Ok(true);
    }
    let curr_value = arr[[rowidx, colidx]];
    let viz_left = arr.slice(s![rowidx, ..colidx]).max()? < &curr_value;
    let viz_right = arr.slice(s![rowidx, (colidx + 1)..]).max()? < &curr_value;
    let viz_above = arr.slice(s![..rowidx, colidx]).max()? < &curr_value;
    let viz_bellow = arr.slice(s![(rowidx + 1).., colidx]).max()? < &curr_value;
    return Ok(viz_left || viz_right || viz_above || viz_bellow);
}

pub fn p8a() -> Result<(), Box<dyn Error>> {
    let mut arr: Array2<u32> = Array::<u32, _>::zeros((99, 99).f());
    let tree_rows = include_str!("../../data/trees.txt").lines();
    for (rowidx, tree_row) in tree_rows.enumerate() {
        for (colidx, tree) in tree_row.chars().enumerate() {
            arr[[rowidx, colidx]] = tree.to_digit(10).unwrap();
        }
    }
    let mut num_viz = 0;
    for rowidx in 0..99 {
        for col in 0..99 {
            let is_viz = is_visible(&arr, rowidx, col)?;
            if is_viz {
                num_viz += 1
            }
        }
    }
    // let file = File::open("data/trees_small.txt")?;
    // let mut reader = ReaderBuilder::new()
    //     .has_headers(false)
    //     .delimiter('')
    //     .from_reader(file);
    // let array_read: Array2<u64> = reader.deserialize_array2((5, 5))?;
    println!("num viz {}", num_viz);
    Ok(())
}
