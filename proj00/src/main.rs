use std::vec::Vec;
use std::io;

// Just a macro to parse lines
macro_rules! parse_line {
    ($separator: literal, $($t: ty),+) => ({
        let mut a_str = String::new();
        match (io::stdin().read_line(&mut a_str)) {
            Ok(_) => {
                a_str.pop();
                let mut a_iter = a_str.split($separator);
                let result = (
                    $(
                        match (a_iter.next()) {
                            Some(value) => match (value.parse::<$t>()) {
                                Ok(v) => Some(v),
                                Err(_) => None,
                            },
                            None => None,
                        },
                    )+
                );
                Ok(result)
            },
            Err(err) => Err(err),
        }
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
    let (Some(num_people), Some(_num_relations)) = parse_line!(",", usize, usize).unwrap() else {panic!()};

    let mut people: Vec<Person> = vec!(Person::new(); num_people);

    while let Ok((Some(person1), Some(person2))) = parse_line!(" ", usize, usize) {
        people.get_mut(person1-1).unwrap().my_friends.push(person2-1);
        people[person2-1].friend_of += 1;
    }

    // Num of People with i friends
    let mut histograma1: Vec<usize> = vec!(0; num_people);
    // Num of People who are friends of i people
    let mut histograma2: Vec<usize> = vec!(0; num_people);

    for p in &people {
        histograma1[p.my_friends.len()] += 1;
        histograma2[p.friend_of] += 1;
    }

    println!("Histograma 1\n{:?}", histograma1);
    println!("Histograma 2\n{:?}", histograma2);

    println!("Output 2\n");
    // Bad O(n³)
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
