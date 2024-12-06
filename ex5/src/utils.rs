use std::{cmp::Ordering, collections::HashMap};

pub fn parse_page_order_rules(rules: &str) -> HashMap<&str, Vec<&str>> {
    let mut page_order_rules: HashMap<&str, Vec<&str>> = HashMap::new();
    for rule in rules.lines() {
        let page_numbers: Vec<&str> = rule.splitn(2, "|").collect();
        if page_numbers.len() != 2 {
            break;
        }
        page_order_rules.entry(page_numbers[0])
            .and_modify(|e| e.push(page_numbers[1]))
            .or_insert(vec![page_numbers[1]]);
    }
    page_order_rules
}

pub fn parse_manual_updates<'a>(updates: &'a str, page_order_rules: HashMap<&'a str, Vec<&'a str>>) -> Vec<Vec<Page<'a>>> {
    updates.lines()
        .map(|l| l.split(",").map(|page| Page{page, page_order_rules: page_order_rules.clone()}).collect())
        .collect()
}

pub struct Page<'a> {
    pub page: &'a str,
    page_order_rules: HashMap<&'a str, Vec<&'a str>>,
}

impl Page<'_> {
    pub fn value(&self) -> usize {
        self.page.parse().expect("Should be a number")
    }
}

impl PartialEq for Page<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.page == other.page
    }
}

impl Eq for Page<'_> { }

impl PartialOrd for Page<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.page == other.page {
            return Some(Ordering::Equal);
        }
        match self.page_order_rules.get(self.page) {
            Some(rules) if rules.contains(&other.page) => Some(Ordering::Less),
            _ => {
                match self.page_order_rules.get(other.page) {
                    Some(rules) if rules.contains(&self.page) => Some(Ordering::Greater),
                    _ => None
                }
            }
        }
    }
}
