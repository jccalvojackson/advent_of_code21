use play_types::Play;
use play_types::Record;
use play_types::Round;
use play_types::RoundResult;
use std::error::Error;
use std::str::FromStr;

mod play_types;

pub fn p2b() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .from_path("data/input2.txt")?;
    let iter = rdr.deserialize();
    let mut total_score: i32 = 0;
    for result in iter {
        let record: Record = result?;
        let oponent_play = Play::from_str(&record.oponent).unwrap();
        let round_result = RoundResult::from_str(&record.mine).unwrap();
        let current_round = Round::from_result(oponent_play, round_result);
        total_score += current_round.total_score();
    }
    println!("total score {}", total_score);
    Ok(())
}
