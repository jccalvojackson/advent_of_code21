use super::utils::push_to_stack;
use super::utils::CrateInstruction;
use std::error::Error;

pub fn p5b() -> Result<(), Box<dyn Error>> {
    // let format = "[{}] {}";
    // let content = "  [1] [2]";
    // let nwhite = count_whitespace_bytes_at_start(content);
    // let (a, b) = scan_fmt_some!(content, format, i32, String);
    // println!("{}, {}- {}", a.unwrap(), b.unwrap(), nwhite);
    let mut crates = include_str!("../../data/crates.txt")
        .split("\n\n")
        .nth(0)
        .expect("nostr")
        .lines()
        .rev();
    //create as many vectors as first line
    crates.next();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
    for line in crates {
        let mut idx = 0;
        push_to_stack(line, &mut stacks, &mut idx).expect("msg");
    }

    // let headers = "action amount conn1 crate_orig conn2 crate_dest";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_path("data/crates_instructions.txt")?;
    let iter = rdr.deserialize();
    for result in iter {
        let record: CrateInstruction = result?;
        let mut tmp_vec: Vec<char> = Vec::new();
        for _ in 0..record.amount {
            let object_to_move = stacks[(record.crate_orig - 1) as usize].pop().unwrap();
            tmp_vec.push(object_to_move)
        }
        for _ in 0..record.amount {
            let el = tmp_vec.pop().unwrap();
            stacks[(record.crate_dest - 1) as usize].push(el);
        }
    }
    for mut v in stacks {
        println!("el {}", v.pop().unwrap());
    }
    Ok(())
}
