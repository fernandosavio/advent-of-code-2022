use std::{fs::File, io::{BufReader, BufRead, Result}};


fn get_biggest(file: File) -> Result<usize> {
    let reader = BufReader::new(file);

    let mut acc: usize = 0;
    let mut biggest: usize = 0;

    for line in reader.lines() {
        if let Ok(value) = line?.parse::<usize>() {
            acc += value;
        } else {
            if acc > biggest {
                biggest = acc;
            }
            acc = 0;
        }
    }


    Ok(biggest)
}

struct Top3 (Vec<usize>);

impl Top3 {
    fn new() -> Self {
        let mut vec = vec![0; 4];
        vec.pop();
        Self (vec)
    }

    fn register(&mut self, num: usize) {
        for (i, &item) in self.0.iter().enumerate() {
            if num > item {
                self.0.insert(i, num);
                self.0.pop();
                return;
            }
        }
    }
}

fn get_top_3(file: File) -> Result<Top3> {
    let reader = BufReader::new(file);

    let mut acc: usize = 0;
    let mut top_3 = Top3::new();

    for line in reader.lines() {
        if let Ok(value) = line?.parse::<usize>() {
            acc += value;
        } else {
            top_3.register(acc);
            acc = 0;
        }
    }

    Ok(top_3)
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let biggest = get_biggest(file)?;
    println!("Biggest: {}", biggest);

    let file = File::open("input.txt")?;
    let top_3 = get_top_3(file)?;
    let total: usize = top_3.0.iter().sum();

    println!("Top 3: {} + {} + {} = {}", top_3.0[0], top_3.0[1], top_3.0[2], total);

    Ok(())
}
