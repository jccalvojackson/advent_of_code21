use csv::Reader;
use std::error::Error;

pub fn p1a() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("data/input_small.txt")?;
    let iter = rdr.deserialize();
    let mut max_value: i32 = 0;
    let mut elf_total_cal: i32 = 0;
    let mut count: i32 = 1;
    let mut count_max_elf: i32 = 1;
    for result in iter {
        let record_str: String = result?;
        println!("{}", record_str);
        (count, max_value, elf_total_cal, count_max_elf) = if record_str.is_empty() {
            (max_value, count_max_elf) = if elf_total_cal > max_value {
                (elf_total_cal, count)
            } else {
                (max_value, count_max_elf)
            };
            (count + 1, max_value, 0, count_max_elf)
        } else {
            let record: i32 = record_str.parse().unwrap();
            (count, max_value, elf_total_cal + record, count_max_elf)
        };
    }
    println!(
        "total inc {}, {}, {}, {}",
        count, max_value, elf_total_cal, count_max_elf
    );
    Ok(())
}
