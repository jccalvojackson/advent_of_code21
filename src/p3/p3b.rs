use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

pub fn p3b() -> Result<(), Box<dyn Error>> {
    let priority_ter = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, char)| (char.clone(), idx as i32 + 1));
    let priority: HashMap<char, i32, RandomState> = HashMap::from_iter(priority_ter);
    println!("value of C is {}", priority.get(&'C').unwrap());

    let mut rucksacks = include_str!("../../data/rucksacks.txt")
        .split("\n")
        .map(|rucksack| {
            let hashset: HashSet<char> = rucksack.chars().collect();
            hashset
        });
    let mut total_priority = 0;
    loop {
        let felffopt = rucksacks.next(); //.unwrap();
        if felffopt.is_none() {
            break;
        }
        let felff = felffopt.unwrap();
        let selff: HashSet<char> = rucksacks.next().unwrap();
        let telff: HashSet<char> = rucksacks.next().unwrap();
        let interfs: HashSet<char> = felff.intersection(&selff).map(|ch| *ch).collect();
        let finter: HashSet<_> = interfs.intersection(&telff).collect();
        let badge = finter.iter().next().unwrap();
        total_priority += priority.get(*badge).unwrap();
    }

    println!("total priority is {}", total_priority);
    Ok(())
}
