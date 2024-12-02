use std::{fs, iter::zip};

fn main() {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let contents = fs::read_to_string("input.txt")
        .expect("Could not read input.txt file");
    
    for line in contents.lines() {
        let mut columns = line.split_ascii_whitespace();
        let num: i32 = columns.next().expect("Should have an item in the first list")
            .parse().expect("First list shoud contain a number");
        list1.push(num);
        let num: i32 = columns.next().expect("Should have an item in the second list")
            .parse().expect("Second list shoud contain a number");
        list2.push(num);
    }

    list1.sort();
    list2.sort();

    let distance = total_distance(&list1, &list2);
    println!("Total distance: {}", distance);

    let similarity = similarity_score(&list1, &list2);
    println!("Similarity: {}", similarity);

}

fn total_distance(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let both_lists = zip(list1.iter(), list2.iter());
    let mut sum = 0;
    for (num1, num2) in both_lists {
        sum += (num1 - num2).abs();
    }
    sum
}

fn similarity_score(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut similarity = 0;
    for num1 in list1.iter() {
        similarity += num1 * list2.iter().filter(|&n| n == num1).count() as i32;
    }
    similarity
}
