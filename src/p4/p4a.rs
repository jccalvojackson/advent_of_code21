use super::utils::interval_to_set;
use super::utils::SectionRecord;
use std::collections::HashSet;
use std::error::Error;

pub fn p4a() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new().from_path("data/section_asignment.txt")?;
    let iter = rdr.deserialize();
    let mut total_iter: i32 = 0;
    for result in iter {
        let record: SectionRecord = result?;
        let finterval: HashSet<i32> = interval_to_set(record.finterval);
        let sinterval: HashSet<i32> = interval_to_set(record.sinterval);
        total_iter += (finterval.is_subset(&sinterval) || sinterval.is_subset(&finterval)) as i32
    }

    println!("total overlapping is {}", total_iter);
    Ok(())
}
