use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

pub fn p3a() -> Result<(), Box<dyn Error>> {
    let priority_ter = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, char)| (char.clone(), idx as i32 + 1));
    let priority: HashMap<char, i32, RandomState> = HashMap::from_iter(priority_ter);
    println!("value of C is {}", priority.get(&'C').unwrap());

    let rucksacks = include_str!("../../data/rucksacks.txt")
        .split("\n")
        .map(|rucksack| {
            let ulim = rucksack.len() / 2;
            let fhalf: HashSet<char> = rucksack[..ulim].chars().collect();
            let shalf: HashSet<char> = rucksack[ulim..].chars().collect();
            let inter: HashSet<_> = fhalf.intersection(&shalf).collect();
            let char_ref = inter.iter().next().unwrap();
            **char_ref
        });
    let mut total_priority = 0;
    for el in rucksacks {
        total_priority += priority.get(&el).unwrap();
    }
    println!("total priority is {}", total_priority);
    Ok(())
}
