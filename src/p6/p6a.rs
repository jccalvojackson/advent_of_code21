use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;

fn is_all_different(deque: &mut VecDeque<char>) -> bool {
    let set_deq: HashSet<char> = deque.iter().map(|ch| *ch).collect();
    set_deq.len() == 14
}

pub fn p6a() -> Result<(), Box<dyn Error>> {
    let stream = include_str!("../../data/stream.txt");
    let mut deque: VecDeque<char> = VecDeque::new();
    let mut init: i32 = 0;
    for (idx, ch) in stream.chars().enumerate() {
        deque.push_back(ch);
        // update_deque(&mut deque, ch);
        if idx < 14 {
            continue;
        }
        deque.pop_front();
        if is_all_different(&mut deque) {
            init = idx as i32;
            break;
        }
    }
    println!("idx {}", init + 1);
    Ok(())
}
