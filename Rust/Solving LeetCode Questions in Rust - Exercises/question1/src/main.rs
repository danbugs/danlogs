fn main() {
    let nums : Vec<i32> = vec![2,7,11,15];
    let target = 9;
    println!("Solution: {:?}", two_sum(nums, target));
}

// APPROACH #1: Brute Force
// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut sol : Vec<i32> = Vec::new();
//     let mut found = false;
//     for (i, &a) in nums.iter().enumerate() {
//         for (j, &b) in nums.iter().enumerate() {
//             let two_sum = a + b;
//             if i != j && two_sum == target {
//                 sol.push(i as i32);
//                 sol.push(j as i32);
//                 found = true;
//                 break;
//             }
//         }
//         if found {
//             break;
//         }
//     }
//     sol
// }

// APPROACH #2: Optimized Solution
// use std::collections::HashMap;
// fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut hm = HashMap::new();
//     for (i, &val) in nums.iter().enumerate() {
//         hm.insert(val, i as i32);
//     }

//     for (i, &val) in nums.iter().enumerate() {
//         let look = target - val;
//         if let Some(&j) = hm.get(&look) {
//             let pos = j as usize;
//             if i != pos {
//                 return vec![i as i32, j];
//             }
//         }
//     }
//     return vec![];
// }

// APPROACH #3: Little Fixes
use std::collections::HashMap;
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = HashMap::with_capacity(nums.len());
    for(i, &val) in nums.iter().enumerate() {
        let look = target - val;
        if let Some(&j) = hm.get(&look) {
            return vec![i as i32, j];
        }
        hm.insert(val, i as i32);
    }
    vec![]
}