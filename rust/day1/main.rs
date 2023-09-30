
use std::{fs, rc::Rc, cell::RefCell};


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
            calories_total: 0
        }
    }

    fn add_calorie(&mut self, calorie: usize) {
        self.elve_calories.push(calorie);
        self.calories_total += calorie;
    }
}

struct ElvesParser {
    highest_elve_index: Option<usize>,
    elves: Rc<RefCell<Vec<Elve>>>,
    calories_ledger: String,
}

impl ElvesParser {
    fn new(ledger_path: &str) -> ElvesParser {
        ElvesParser {
            elves: Rc::new(RefCell::new(vec![])),
            highest_elve_index: None,
            calories_ledger: fs::read_to_string(ledger_path).expect("Couldn't read calories file")
        }
    }

    fn parse_elves(& mut self) {
        for line in self.calories_ledger.lines() {

            // Create new elve if not present
            if line.trim().is_empty() {
                // check if total is higher than highest and update highest elve
                if let Some(active_elve) = Rc::clone(&self.elves).borrow().last() {
                    if let Some(current_highest_elve_index) = self.highest_elve_index {
                        let current_highest_elve = &self.elves.borrow()[current_highest_elve_index];
                        if active_elve.calories_total > current_highest_elve.calories_total {
                            println!("Highest elve {:#?}", current_highest_elve);
                        }
                        println!("Highest elve {:#?}", current_highest_elve);
                    } else {
                        self.highest_elve_index = Some(Rc::clone(&self.elves).borrow().len() - 1);
                    }
                }

                Rc::clone(&self.elves).borrow_mut().push(Elve::new((Rc::clone(&self.elves).borrow().len() as u16) + 1));
                continue;
            }
            
            // populate calorie for active elve
            if let Some(last_elve) = Rc::clone(&self.elves).borrow_mut().last_mut() {
                last_elve.add_calorie(
                    line.trim().parse().expect("Calorie not a valid number")
                )
            } 
        }
        
        let highest_elve = &self.elves.borrow()[self.highest_elve_index.expect("No index")];
        println!("{:#?} {}", highest_elve, highest_elve.elve_index);
    }
}


fn main() {
    let mut elves_parser = ElvesParser::new("day1/calories.txt");
    elves_parser.parse_elves();

    println!("Elve solution impl!")
}

