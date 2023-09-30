use std::{cell::RefCell, fs};

#[derive(Debug)]
struct Elve {
    elve_index: u16,
    elve_calories: Vec<usize>,
    calories_total: usize,
}

impl Elve {
    fn new(index: u16) -> Elve {
        Elve {
            elve_index: index,
            elve_calories: vec![],
            calories_total: 0,
        }
    }

    fn add_calorie(&mut self, calorie: usize) {
        self.elve_calories.push(calorie);
        self.calories_total += calorie;
    }
}

struct ElvesParser {
    highest_elve_vec_index: Option<usize>,
    elves: RefCell<Vec<Elve>>,
    calories_ledger: String,
}

impl ElvesParser {
    fn new(ledger_path: &str) -> ElvesParser {
        ElvesParser {
            elves: RefCell::new(vec![]),
            highest_elve_vec_index: None,
            calories_ledger: fs::read_to_string(ledger_path).expect("Couldn't read calories file"),
        }
    }

    fn parse_elves(&mut self) {
        // Create first elve
        self.elves.borrow_mut().push(Elve::new(1));
        for line in self.calories_ledger.lines() {
            // Create new elve if not present
            if line.trim().is_empty() {
                // check if total is higher than highest and update highest elve
                if let Some(active_elve) = self.elves.borrow().last() {
                    println!("New elve: {:?}", active_elve);

                    if let Some(current_highest_elve_vec_index) = self.highest_elve_vec_index {
                        let current_highest_elve = &self.elves.borrow()[current_highest_elve_vec_index];
                        if active_elve.calories_total > current_highest_elve.calories_total {
                            self.highest_elve_vec_index = Some(self.elves.borrow().len() - 1);
                        }
                    } else {
                        self.highest_elve_vec_index = Some(self.elves.borrow().len() - 1);
                    }
                }

                let num_of_elves = (self.elves.borrow().len() as u16) + 1;

                self.elves.borrow_mut().push(Elve::new(num_of_elves));

                continue;
            }

            // populate calorie for active elve
            if let Some(last_elve) = self.elves.borrow_mut().last_mut() {
                last_elve.add_calorie(line.trim().parse().expect("Calorie not a valid number"))
            }
        }
    }
}

fn main() {
    let mut elves_parser = ElvesParser::new("day1/calories.txt");
    elves_parser.parse_elves();

    println!(
        "Highest elve recorded {:#?}",
        elves_parser.elves.borrow()[elves_parser.highest_elve_vec_index.expect("No index")]
    );

    println!("Elve solution impl!")
}
