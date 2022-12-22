use ndarray::{s, Array2};

use std::error::Error;

enum MyIndex {
    Rangeto(usize),
    Rangefrom(usize),
    Idx(usize),
}

pub fn scenic_score(
    arr: &Array2<u32>,
    rowidx: usize,
    colidx: usize,
    size: usize,
) -> Result<i32, Box<dyn Error>> {
    if colidx == 0 || colidx == (size - 1) || rowidx == 0 || rowidx == (size - 1) {
        return Ok(0);
    }
    let curr_value = arr[[rowidx, colidx]];
    let ld = get_idx_first_ge(
        arr,
        MyIndex::Idx(rowidx),
        MyIndex::Rangeto(colidx),
        curr_value,
    )? + 1;
    let rd = get_idx_first_ge(
        arr,
        MyIndex::Idx(rowidx),
        MyIndex::Rangefrom(colidx + 1),
        curr_value,
    )? + 1;
    let ad = get_idx_first_ge(
        arr,
        MyIndex::Rangeto(rowidx),
        MyIndex::Idx(colidx),
        curr_value,
    )? + 1;
    let bd = get_idx_first_ge(
        arr,
        MyIndex::Rangefrom(rowidx + 1),
        MyIndex::Idx(colidx),
        curr_value,
    )? + 1;
    return Ok((ld * rd * ad * bd) as i32);
}

fn get_idx_first_ge(
    arr: &Array2<u32>,
    rowidx: MyIndex,
    colidx: MyIndex,
    cv: u32,
) -> Result<usize, Box<dyn Error>> {
    let mut ans = 0;
    let sz = match (rowidx, colidx) {
        (MyIndex::Rangeto(rangeto), MyIndex::Idx(idx)) => arr.slice(s![..rangeto;-1, idx]),
        (MyIndex::Rangefrom(rangefrom), MyIndex::Idx(idx)) => arr.slice(s![rangefrom.., idx]),
        (MyIndex::Idx(idx), MyIndex::Rangeto(rangeto)) => arr.slice(s![idx, ..rangeto;-1]),
        (MyIndex::Idx(idx), MyIndex::Rangefrom(rangefrom)) => arr.slice(s![idx, rangefrom..]),
        _ => arr.slice(s![..0, 0]),
    };
    for (idx, el) in sz.iter().enumerate() {
        ans = idx;
        if el >= &cv {
            break;
        }
    }
    Ok(ans)
}
