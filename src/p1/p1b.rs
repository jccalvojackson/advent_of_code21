use std::collections::BinaryHeap;
use std::error::Error;

pub fn p1b() -> Result<(), Box<dyn Error>> {
    let elfs = include_str!("../../data/input.txt")
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .fold(0, |acc, i| acc + i.parse::<i32>().unwrap())
        });
    let mut heap = BinaryHeap::new();
    elfs.for_each(|elf_sum| heap.push(elf_sum));
    let mut top_3_sum = 0;
    for _ in 0..3 {
        if let Some(v) = heap.pop() {
            top_3_sum += v;
        }
    }
    println!("The top 3 have {} calories", top_3_sum);
    Ok(())
}
