use std::{fs::File, io::{BufReader, BufRead, Result as IOResult}};

#[derive(Clone, Copy)]
enum AdversaryMoves { Rock, Paper, Scissors }

impl TryFrom<&u8> for AdversaryMoves {
    type Error = &'static str;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            b'A' => Ok(Self::Rock),
            b'B' => Ok(Self::Paper),
            b'C' => Ok(Self::Scissors),
            _ => Err("Invalid adversary move"),
        }
    }
}

enum MyMoves { Rock, Paper, Scissors }

impl TryFrom<&u8> for MyMoves {
    type Error = &'static str;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            b'X' => Ok(Self::Rock),
            b'Y' => Ok(Self::Paper),
            b'Z' => Ok(Self::Scissors),
            _ => Err("Invalid move"),
        }
    }
}

impl MyMoves {
    fn as_usize(&self) -> usize {
        match self {
            MyMoves::Rock => 1_usize,
            MyMoves::Paper => 2_usize,
            MyMoves::Scissors => 3_usize,
        }
    }
}

enum Strategy { Lose, Draw, Win }

impl TryFrom<&u8> for Strategy {
    type Error = &'static str;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            b'X' => Ok(Self::Lose),
            b'Y' => Ok(Self::Draw),
            b'Z' => Ok(Self::Win),
            _ => Err("Invalid move"),
        }
    }
}


impl Strategy {
    fn calculate_move(&self, &adv: &AdversaryMoves) -> MyMoves {
        match self {
            Self::Draw => match adv {
                AdversaryMoves::Rock => MyMoves::Rock,
                AdversaryMoves::Paper => MyMoves::Paper,
                AdversaryMoves::Scissors => MyMoves::Scissors,
            },
            Self::Win => match adv {
                AdversaryMoves::Rock => MyMoves::Paper,
                AdversaryMoves::Paper => MyMoves::Scissors,
                AdversaryMoves::Scissors => MyMoves::Rock,
            },
            Self::Lose => match adv {
                AdversaryMoves::Rock => MyMoves::Scissors,
                AdversaryMoves::Paper => MyMoves::Rock,
                AdversaryMoves::Scissors => MyMoves::Paper,
            },
        }
    }
}

fn part_one() -> IOResult<usize> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_points = 0_usize;

    for line in reader.lines() {
        let moves = line.unwrap();

        let bytes = moves.as_bytes();
        let adv_move_char = bytes.first().unwrap();
        let my_move_char = bytes.last().unwrap();

        let my_move = MyMoves::try_from(my_move_char).unwrap();
        let adv_move = AdversaryMoves::try_from(adv_move_char).unwrap();

        let x: usize = my_move.as_usize();

        total_points += x;

        total_points += match (my_move, adv_move) {
            (MyMoves::Paper, AdversaryMoves::Rock)
            | (MyMoves::Scissors, AdversaryMoves::Paper)
            | (MyMoves::Rock, AdversaryMoves::Scissors) => 6,
            (MyMoves::Rock, AdversaryMoves::Rock)
            | (MyMoves::Paper, AdversaryMoves::Paper)
            | (MyMoves::Scissors, AdversaryMoves::Scissors) => 3,
            _ => 0,
        };
    }

    Ok(total_points)
}

fn part_two() -> IOResult<usize> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_points = 0_usize;

    for line in reader.lines() {
        let moves = line.unwrap();

        let bytes = moves.as_bytes();
        let adv_move_char = bytes.first().unwrap();
        let strategy_char = bytes.last().unwrap();

        let adv_move = AdversaryMoves::try_from(adv_move_char).unwrap();
        let strategy = Strategy::try_from(strategy_char).unwrap();
        let my_move = strategy.calculate_move(&adv_move);

        let x: usize = my_move.as_usize();

        total_points += x;

        total_points += match (my_move, adv_move) {
            (MyMoves::Paper, AdversaryMoves::Rock)
            | (MyMoves::Scissors, AdversaryMoves::Paper)
            | (MyMoves::Rock, AdversaryMoves::Scissors) => 6,
            (MyMoves::Rock, AdversaryMoves::Rock)
            | (MyMoves::Paper, AdversaryMoves::Paper)
            | (MyMoves::Scissors, AdversaryMoves::Scissors) => 3,
            _ => 0,
        };
    }

    Ok(total_points)
}

fn main() -> IOResult<()> {

    let total_points = part_one()?;
    println!("Total points: {}", total_points);

    let total_points = part_two()?;
    println!("Total points: {}", total_points);

    Ok(())
}
