use std::{collections::HashMap, fs};

mod utils;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should be able to read input");

    let sections: Vec<&str> = contents.splitn(2, "\n\n").collect();

    let page_order_rules: HashMap<&str, Vec<&str>> = utils::parse_page_order_rules(sections[0]);

    let updates: Vec<Vec<utils::Page>> = utils::parse_manual_updates(sections[1], page_order_rules.clone());

    let mut sum: usize = 0;
    let mut sum_invalid: usize = 0;
    for mut update in updates {
        let mut valid = true;
        for (i, page) in update.iter().enumerate() {
            match page_order_rules.get(page.page) {
                None => { continue; },
                Some(pages) => {
                    if update[0..i].iter().any(|p| pages.contains(&p.page)) {
                        valid = false;
                        break;
                    }
                },
            }
        }
        if valid {
            sum += update[update.len() / 2].value();
        } else {
            // This can panic in some cases if any 2 pages in the update
            // are not covered by a page ordering rule.
            // It worked for my input, but the problem statement does not
            // guarantee that the rules are exhaustive.
            update.sort_by(|a, b| a.partial_cmp(b).expect("Oups"));
            sum_invalid += update[update.len() / 2].value();
        }
    }
    println!("Sum: {sum}");
    println!("Sum invalid: {sum_invalid}");
}
