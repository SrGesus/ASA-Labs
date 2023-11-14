use std::vec::Vec;
use std::io::{self, BufReader, BufRead};

macro_rules! parse_line {
    ($separator: literal, $($t: ty),+) => ({
        let mut a_str = String::new();
        io::stdin().read_line(&mut a_str).expect("read error");
        a_str.pop();
        let mut a_iter = a_str.split($separator);
        (
            $(
                a_iter.next().unwrap().parse::<$t>().expect("parse error"),
            )+
        )
    })
}

#[derive(Clone, Debug)]
struct Person {
    my_friends: Vec<usize>,
    friend_of: usize,
}

impl Person {
    pub fn new() -> Self {
        Self {
            my_friends: Vec::new(),
            friend_of: 0,
        }
    }
}

fn main() {
    let (num_people, _num_relations) = parse_line!(",", usize, usize);

    let mut people: Vec<Person> = vec!(Person::new(); num_people);
    
    for line in BufReader::new(io::stdin()).lines().map(|l| l.unwrap()) {
        let mut data = line.split(" ");
        let person1 = data.next().unwrap().parse::<usize>().unwrap();
        let person2 = data.next().unwrap().parse::<usize>().unwrap();
        people.get_mut(person1-1).unwrap().my_friends.push(person2-1);
        people[person2-1].friend_of += 1;
    }

    // Num Pessoas com i amigas
    let mut histograma1: Vec<usize> = vec!(0; num_people);
    // Num Pessoas que são amigas de i pessoas
    let mut histograma2: Vec<usize> = vec!(0; num_people);

    for p in &people {
        histograma1[p.my_friends.len()] += 1;
        histograma2[p.friend_of] += 1;
    }

    println!("Histograma 1\n{:?}", histograma1);
    println!("Histograma 2\n{:?}", histograma2);

    println!("Output 2\n");
    for p in &people {
        for p2 in &people {
            let mut common = 0;
            for friend in &p.my_friends {
                if p2.my_friends.contains(&friend) {
                    common += 1;
                }
            }
            print!("{} ",common);
        }
        println!("");
    }

}
