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

    let mut i: usize = 0;
    let mut sum: i32 = 0;
    let len = indexes.len();

    while i < len && i < k {
        sum = sum + (*array)[indexes[i]];
        i = i + 1;
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

    if let Some(value) = indexes.iter().max() {
        if (*value) > (array.len()-1) {
            let msg = format!("Irregular indexes: {:?}", *indexes);
            panic!(msg);
            return 0;
        }
    }

    if let Some(value) = indexes.iter().min() {
        if (*value) < 0 {
            let msg = format!("Irregular indexes: {:?}", *indexes);
            panic!(msg);
            return 0;
        }
    }

    let mut i: usize = indexes.len() - 1;
    if i < 0 {
        return 0;
    }

    let mut sum: i32 = 0;

    let mut count: usize = 0;
    while i >= 0 && count < k {
        sum = sum + (*array)[indexes[i]];
        i = i - 1;
        count = count + 1;
    }

    return sum;
}

fn searcher(
    considering_index: usize,
    k_value: usize,
    options_indexes: &mut Vec<usize>,
    retained_indexes: &mut Vec<usize>,
    dropped_indexes: &mut Vec<usize>,
    input_array: &Vec<i32>
) -> Vec<(usize, usize, usize, usize)> {

    

    return vec![(0, 1, 2, 3)];
}

fn main() {

    let mut input: Vec<i32> = vec![1, 0, -1, 0, -2, 2];
    input.sort(); // After sorting: [-2, -1, 0, 0, 1, 2]

    let target: i32 = 0;
    let k_value: i32 = 4;

    println!("Input array is: {:?}", input);
    println!("Number of items in input array: {}", input.len());
    println!("Number of items in each solution: {}", k_value);

    let min = input.iter().min();
    if let Some(value) = min {
        println!("Min value is: {}", value);
    }

    let max = input.iter().max();
    if let Some(value) = max {
        println!("Max value is: {}", value);
    }

    println!("Target is: {}", target);

    let root = Node {
        id: "node0".to_string(),
        k_value: 4,
        min_composition: -3,
        max_composition: -3,
        options_indexes: &mut vec![0, 1, 2, 3, 4, 5],
        retained_indexes: &mut vec![],
        dropped_indexes: &mut vec![],
        input: &vec![-2, -1, 0, 0, 1, 2],
        left_node: Box::new(BranchNode(Node {
            id: "node0".to_string(),
            k_value: 4,
            min_composition: -3,
            max_composition: -3,
            options_indexes: &mut vec![0, 1, 2, 3, 4, 5],
            retained_indexes: &mut vec![],
            dropped_indexes: &mut vec![],
            input: &vec![-2, -1, 0, 0, 1, 2],
            left_node: Box::new(Nil),
            right_node: Box::new(Nil)
        })),
        right_node: Box::new(BranchNode(Node {
            id: "node0".to_string(),
            k_value: 4,
            min_composition: -3,
            max_composition: -3,
            options_indexes: &mut vec![0, 1, 2, 3, 4, 5],
            retained_indexes: &mut vec![],
            dropped_indexes: &mut vec![],
            input: &vec![-2, -1, 0, 0, 1, 2],
            left_node: Box::new(Nil),
            right_node: Box::new(Nil)
        }))
    };

}
