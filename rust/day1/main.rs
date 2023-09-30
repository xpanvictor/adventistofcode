/// Goal of this implementation is to use as much rust features as possible
/// while ensuring a simple implementation of the algorithm


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
    next_two_elve_vec_indices: (usize, usize)
}

impl ElvesParser {
    fn new(ledger_path: &str) -> ElvesParser {
        ElvesParser {
            elves: RefCell::new(vec![]),
            highest_elve_vec_index: None,
            calories_ledger: fs::read_to_string(ledger_path).expect("Couldn't read calories file"),
            next_two_elve_vec_indices: (0, 0)
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
                    let active_elve_index = self.elves.borrow().len() - 1;

                    let second_highest_elve_vec_index = self.next_two_elve_vec_indices.0;
                    let second_highest_elve = &self.elves.borrow()[second_highest_elve_vec_index];
                    let third_highest_elve_vec_index = self.next_two_elve_vec_indices.1;
                    let third_highest_elve = &self.elves.borrow()[third_highest_elve_vec_index];

                    if let Some(current_highest_elve_vec_index) = self.highest_elve_vec_index {
                        let current_highest_elve = &self.elves.borrow()[current_highest_elve_vec_index];
                        if active_elve.calories_total > current_highest_elve.calories_total {
                            self.highest_elve_vec_index = Some(active_elve_index);
                            self.next_two_elve_vec_indices.0 = current_highest_elve_vec_index;
                            self.next_two_elve_vec_indices.1 = second_highest_elve_vec_index;
                        } else if active_elve.calories_total > second_highest_elve.calories_total {
                            self.next_two_elve_vec_indices.0 = active_elve_index;
                            self.next_two_elve_vec_indices.1 = second_highest_elve_vec_index;
                        } else if active_elve.calories_total > third_highest_elve.calories_total {
                            self.next_two_elve_vec_indices.1 = active_elve_index;
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

    fn sum_top_three_elves_calories(&self) -> usize {
        let highest_elve = &self.elves.borrow()[self.highest_elve_vec_index.expect("No index")];
        let second_highest_elve = &self.elves.borrow()[self.next_two_elve_vec_indices.0];
        println!(
            "Second highest elve recorded {:#?}",
            second_highest_elve
        );
        let third_highest_elve = &self.elves.borrow()[self.next_two_elve_vec_indices.1];
        println!(
            "Third highest elve recorded {:#?}",
            third_highest_elve
        );

        highest_elve.calories_total
                + second_highest_elve.calories_total 
                + third_highest_elve.calories_total
    }
}

fn main() {
    let mut elves_parser = ElvesParser::new("day1/calories.txt");
    elves_parser.parse_elves();

    println!(
        "Highest elve recorded {:#?}",
        elves_parser.elves.borrow()[elves_parser.highest_elve_vec_index.expect("No index")]
    );

    let total_top_three_calories = elves_parser.sum_top_three_elves_calories();
    println!("The top three elves calories total is {total_top_three_calories}");

    println!("Elve solution impl!")
}
