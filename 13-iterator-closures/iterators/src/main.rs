fn main() {
    let v1 : Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let total: i32 = v1.iter().sum();
    println!("Total: {}", total);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    for val in v2.iter() {
        println!("Got: {}", val);
    }

    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") }
    ];
    
    let in_my_size = shoes_in_my_size(shoes, 10);

    for shoe in in_my_size.iter() {
        println!("{:?}", shoe);
    }

    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                    .map(|(a, b)| a * b)
                    .filter(|x| x % 3 == 0)
                    .sum();

    println!("{}", sum);
    
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoes_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoes_size )
        .collect()
}

struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}