struct Node<'a> {
    id: String,
    k_value: usize,
    min_composition: i32,
    max_composition: i32,
    options_indexes: &'a mut Vec<usize>,
    retained_indexes: &'a mut Vec<usize>,
    dropped_indexes: &'a mut Vec<usize>,
    input: &'a Vec<i32>,
    left_node: Box<Branch<'a>>,
    right_node: Box<Branch<'a>>
}

enum Branch<'a> {
    BranchNode(Node<'a>),
    Nil
}

use crate::Branch::{BranchNode, Nil};

// Input:
// array = [-2, -1, 0, 0, 1, 2]
// indexes = [0, 2, 3, 5] 
// k = 3
//
// Compute:
// sum = array[0] + array[2] + array[3] = -2 + 0 + 0 = -2
//
// Output:
// sum = -2
//
// Test:
// let test_array_1: Vec<i32> = vec![-2, -1, 0, 0, 1, 2];
// let test_indexes_1: Vec<usize> = vec![0, 2, 3, 5];
// let test_k_value_1: usize = 3;
// let test_result_value_1 = left_sum_by_indexes(&test_array_1, &test_indexes_1, test_k_value_1);
// assert_eq!(test_result_value_1, -2);
fn left_sum_by_indexes(array: &Vec<i32>, indexes: &Vec<usize>, k: usize) -> i32 {

    // Bound check
    if let Some(value) = indexes.iter().max() {
        if (*value) > (array.len()-1) {
            let msg = format!("Irregular indexes: {:?}", *indexes);
            panic!(msg);
        }
    }

    let mut sum: i32 = 0;
    let mut count: usize = indexes.len().min(k);
    let mut index: usize = 0;

    while count > 0 {
        sum = sum + array[indexes[index]];

        count = count - 1;
        index = index + 1;
    }

    return sum;
}

// Input:
// array = [-2, -1, 0, 0, 1, 2, 4]
// indexes = [0, 1, 4, 5, 6] 
// k = 2
//
// Compute:
// sum = array[6] + array[5] = 4 + 2 = 6
//
// Output:
// sum = 6
//
// Test:
// let test_array_2: Vec<i32> = vec![-2, -1, 0, 0, 1, 2, 4];
// let test_indexes_2: Vec<usize> = vec![0, 1, 4, 5, 6];
// let test_k_value_2: usize = 2;
// let test_result_value_2 = right_sum_by_indexes(&test_array_2, &test_indexes_2, test_k_value_2);
// assert_eq!(test_result_value_2, 6);
fn right_sum_by_indexes(array: &Vec<i32>, indexes: &Vec<usize>, k: usize) -> i32 {

    // Bound check
    if let Some(value) = indexes.iter().max() {
        if (*value) > (array.len()-1) {
            let msg = format!("Irregular indexes: {:?}", *indexes);
            panic!(msg);
        }
    }

    let mut sum: i32 = 0;
    let mut count: usize = indexes.len().min(k);
    let mut index: usize = match indexes.len() {
        0 => 0,
        _ => indexes.len() - 1
    };
    while count > 0 {
        sum = sum + array[indexes[index]];
        count = count - 1;

        if index > 0 {
            index = index - 1;
        }
    }

    return sum;
}

// Input:
// array = [3, 2, 8, 7, 6]
// indexes = [0, 2, 3]
// 
// Compute:
// sum = array[0] + array[2] + array[3] = 3 + 8 + 7 = 18
//
// Output:
// sum = 18
//
// Test:
// let test_array: Vec<i32> = vec![3, 2, 8, 7, 6];
// let test_indexes: Vec<usize> = vec![0, 2, 3];
// let test_result = sum_by_indexes(&test_array, &test_indexes);
// assert_eq!(test_result, 3 + 8 + 7);
fn sum_by_indexes(array: &Vec<i32>, indexes: &Vec<usize>) -> i32 {
    let mut sum: i32 = 0;
    for index in indexes {
        sum = sum + array[*index];
    }

    return sum;
}

fn searcher(
    options: &mut Vec<usize>,
    retained: &mut Vec<usize>,
    dropped: &mut Vec<usize>,
    input: &Vec<i32>,
    round: &mut u32,
    k: usize,
    target: i32,
    solutions: &mut Vec<(usize, usize, usize, usize)>
) {

    println!("Iter: {}", *round);
    println!("Options: {:?}", options);
    println!("Retained: {:?}", retained);
    println!("Dropped: {:?}", dropped);
    println!("Input: {:?}", input);
    println!("Target: {:?}", target);

    let current_k = k - retained.len();
    let retained_sum = sum_by_indexes(input, retained);
    let min: i32 = retained_sum + left_sum_by_indexes(input, options, current_k);
    let max: i32 = retained_sum + right_sum_by_indexes(input, options, current_k);

    println!("Min: {}", min);
    println!("Max: {}", max);

    if current_k == 0 {
        // Say that We've already collected enough items
        // Let's check if it satisfies ...

        if target == retained_sum {
            // When it satisfies ...
            // Then we push it into the solution set

            solutions.push((retained[0], retained[1], retained[2], retained[3]));
        }

        return;

    }

    // We still need collect more items

    // First We check that 
    // if there exit(s) solution(s) under current context ...

    if (target < min) || (target > max) {
        // This indicates that it won't be possible 
        // to find any solution if We branching in.

        // So cutting off.

        return;

    }

    // It's still possible 
    // to find (some) solution(s) in left or/and right branch

    // Make sure that there be still items in option set
    if options.len() < 1 {
        return;
    }

    // Start making left branch ...
    println!("Left branching ...");

    let considering: usize = options.remove(0);
    retained.push(considering);

    *round = *round + 1;
    searcher(options, retained, dropped, input, round, k, target, solutions);

    // After the left branch returned, make right branch ...
    println!("Right branching ...");

    let considering_opt: Option<usize> = retained.pop();
    if let Some(usize_value) = considering_opt {
        dropped.push(usize_value);
        *round = *round + 1;
        searcher(options, retained, dropped, input, round, k, target, solutions);
    }

    // After the right branch returned, 
    // should restore the scene for the parent branch
    println!("Returning ...");

    let considering_opt: Option<usize> = dropped.pop();
    if let Some(usize_value) = considering_opt {
        options.insert(0, usize_value);
    }

}

fn main() {

    let mut input: Vec<i32> = vec![1, 0, -1, 0, -2, 2];
    input.sort(); // After sorting: [-2, -1, 0, 0, 1, 2]

    let target: i32 = 0;
    let k_value: usize = 4;

    println!("Input array is: {:?}", input);
    println!("Number of items in input array: {}", input.len());
    println!("Number of items in each solution: {}", k_value);

    let mut options: Vec<usize> = Vec::new();
    let mut i: usize = 0;
    while i < input.len() {
        options.push(i);
        i = i + 1;
    }

    let mut retained: Vec<usize> = Vec::new();
    let mut dropped: Vec<usize> = Vec::new();

    let mut solutions: Vec<(usize, usize, usize, usize)> = Vec::new();
    let mut round: u32 = 0;

    searcher(&mut options, &mut retained, &mut dropped, &input, &mut round, k_value, target, &mut solutions);

    println!("Solutions: {:?}", solutions);

}
