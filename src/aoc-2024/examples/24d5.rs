use std::fs;

fn main() {
    let raw_input = fs::read("inputs/24d5.txt").expect("Something went wrong reading the file");
    let raw_input = String::from_utf8(raw_input).unwrap();
    let (rules, pages) = raw_input.split_once("\n\n").unwrap();
    let rules = Rules::from_raw(rules);
    let mut manual = Manual::from_raw(pages);
    let mut manual2 = manual.clone();
    manual.filter_updates(&rules);
    let result = manual.sum_middle();
    println!("Result1: {}", result);

    manual2.keep_bad_updates(&rules);
    manual2.enforce_rules(&rules);
    let result = manual2.sum_middle();
    println!("Result2: {}", result);
}

struct Rules<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> Rules<'a> {
    fn from_raw(raw: &'a str) -> Self {
        let rules = raw
            .lines()
            .map(|line| {
                let (key, value) = line.split_once("|").unwrap();
                Rule {
                    before: key,
                    after: value,
                }
            })
            .collect();
        Self { rules }
    }

    fn check(&self, update: &Update) -> bool {
        self.rules.iter().all(|rule| rule.check(update))
    }

    fn enforce(&self, update: &mut Update) {
        let mut max_tries = update.pages.len();
        while max_tries > 0 {
            self.rules.iter().for_each(|rule| rule.enforce(update));
            if self.check(update) {
                return;
            }
            max_tries -= 1;
        }
        panic!("Could not enforce rules");
    }
}

struct Rule<'a> {
    before: &'a str,
    after: &'a str,
}

impl Rule<'_> {
    fn check(&self, update: &Update) -> bool {
        //check if rule applies
        if !update.pages.contains(&self.before) || !update.pages.contains(&self.after) {
            return true;
        }
        //check if rule is respected
        let before_idx = update.pages.iter().position(|&x| x == self.before).unwrap();
        let after_idx = update.pages.iter().position(|&x| x == self.after).unwrap();
        before_idx < after_idx
    }

    fn enforce(&self, update: &mut Update) {
        if self.check(update) {
            return;
        }
        let before_idx = update.pages.iter().position(|&x| x == self.before).unwrap();
        let after_idx = update.pages.iter().position(|&x| x == self.after).unwrap();
        update.pages.swap(before_idx, after_idx);
    }
}

#[derive(Clone)]
struct Update<'a> {
    pages: Vec<&'a str>,
}

impl<'a> Update<'a> {}

#[derive(Clone)]
struct Manual<'a> {
    updates: Vec<Update<'a>>,
}

impl<'a> Manual<'a> {
    fn from_raw(raw: &'a str) -> Self {
        let updates = raw
            .lines()
            .map(|line| {
                let pages = line.split(",").collect();
                Update { pages }
            })
            .collect();
        Self { updates }
    }

    fn filter_updates(&mut self, rules: &Rules) {
        self.updates.retain(|update| rules.check(update));
    }

    fn keep_bad_updates(&mut self, rules: &Rules) {
        self.updates.retain(|update| !rules.check(update));
    }

    fn enforce_rules(&mut self, rules: &Rules) {
        self.updates
            .iter_mut()
            .for_each(|update| rules.enforce(update));
    }

    fn sum_middle(&self) -> usize {
        let mut acc = 0;
        for update in &self.updates {
            let len = update.pages.len();
            acc += update.pages[len / 2].parse::<usize>().unwrap();
        }
        acc
    }
}
