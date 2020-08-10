// Test:

struct DOTDrawer {
    graphviz_dot_string: String,
    started: bool,
    finished: bool
}

impl DOTDrawer {

    pub fn new() -> Self {
        let dot_string = String::from("");
        return DOTDrawer {
            graphviz_dot_string: dot_string,
            started: false,
            finished: false
        };
    }

    fn append_string(&mut self, string_to_append: &str) {
        self.graphviz_dot_string = format!("{}{}", self.graphviz_dot_string, string_to_append);
    }

    pub fn start(&mut self) {
        if !self.started {
            self.append_string("digraph G {node[shape = record];");
            self.started = true;
        }
    }

    pub fn add_node(&mut self, id: &str, min: i32, max: i32, options: &Vec<usize>, retained: &Vec<usize>, dropped: &Vec<usize>, input: &Vec<i32>, sealed: bool) {
        if (!self.finished) && (self.started) {
            let id_string = id;
            let node_string_content = format!("label = \" {{ {{ id | {} }} | {{ min | {} }} | {{ max | {} }} | {{ options | {:?} }} | {{ retained | {:?} }} | {{ dropped | {:?} }} | {{ input | {:?} }} }}\"", id_string, min, max, options, retained, dropped, input);
            let sealed_sign = match sealed {
                true => ", style = filled",
                false => ""
            };
            let node_string = format!("{} [{}{}];", id_string, node_string_content, sealed_sign);
            
            self.append_string(&node_string);
        }
    }

    pub fn root(&mut self, id: &str, min: i32, max: i32, options: &Vec<usize>, retained: &Vec<usize>, dropped: &Vec<usize>, input: &Vec<i32>, sealed: bool) {
        self.add_node(id, min, max, options, retained, dropped, input, sealed);
    }

    pub fn connect(&mut self, from_id: &str, to_id: &str, connect_type: &str) {
        let connect_string = format!("{} -> {} [label = \"{}\"]; ", from_id, to_id, connect_type);
        self.append_string(&connect_string);
    }

    pub fn end(&mut self) {
        if !self.finished {
            self.append_string("}");
            self.finished = true;
        }
    }

    pub fn print(&self) {
        println!("{}", self.graphviz_dot_string);
    }
}

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

    // searcher(&mut options, &mut retained, &mut dropped, &input, &mut round, k_value, target, &mut solutions);

    // println!("Solutions: {:?}", solutions);

    let mut dot_drawer = DOTDrawer::new();
    dot_drawer.start();

    let test_id_0 = String::from("node0");
    let test_id_1 = String::from("node1");
    let test_id_2 = String::from("node2");
    let test_min: i32 = -3;
    let test_max: i32 = -3;
    let test_options = vec![0, 1, 2, 3, 4, 5];
    let test_retained = vec![0, 1];
    let test_dropped = vec![0, 1, 2, 3];
    let test_input = vec![1, 2, 3, 4];

    dot_drawer.root(&test_id_0, test_min, test_max, &test_options, &test_retained, &test_dropped, &test_input, false);
    dot_drawer.add_node(&test_id_1, test_min, test_max, &test_options, &test_retained, &test_dropped, &test_input, false);
    dot_drawer.add_node(&test_id_2, test_min, test_max, &test_options, &test_retained, &test_dropped, &test_input, true);
    dot_drawer.connect(&test_id_0, &test_id_1, "in");
    dot_drawer.connect(&test_id_0, &test_id_2, "out");
    dot_drawer.end();
    dot_drawer.print();

}
