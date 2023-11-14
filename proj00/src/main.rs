use std::vec::Vec;
use std::io::{self, BufReader, BufRead};

#[derive(Clone, Debug)]
struct Person {
    my_friends: Vec<usize>,
    friend_of: Vec<usize>,
}

impl Person {
    pub fn new() -> Self {
        Self {
            my_friends: Vec::new(),
            friend_of: Vec::new(),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    let line1 = reader.next().unwrap().unwrap();
    let mut line1 = line1.split(",");
    let num_people = line1.next().unwrap().parse::<usize>().unwrap();
    let num_relations = line1.next().unwrap().parse::<usize>().unwrap();

    let mut people: Vec<Person> = vec!(Person::new(); num_people);

    for line in reader.map(|l| l.unwrap()) {
        let mut data = line.split(" ");
        let person1 = data.next().unwrap().parse::<usize>().unwrap();
        let person2 = data.next().unwrap().parse::<usize>().unwrap();
        people.get_mut(person1-1).unwrap().my_friends.push(person2-1);
        people.get_mut(person2-1).unwrap().friend_of.push(person1-1);
    }

    // Pessoas com i amigas
    let mut histograma1: Vec<usize> = vec!(0; num_people);
    // Pessoas que são amigas de i pessoas
    let mut histograma2: Vec<usize> = vec!(0; num_people);

    for p in people {
        histograma1[p.my_friends.len()] = histograma1[p.my_friends.len()]+1;
        histograma2[p.friend_of.len()] = histograma2[p.friend_of.len()]+1;
    }

    println!("Histograma 1\n{:?}", histograma1);
    println!("Histograma 2\n{:?}", histograma2);

}
