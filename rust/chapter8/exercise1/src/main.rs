/*
Given a list of integers, 
use a vector and return the mean (the average value), 
median (when sorted, the value in the middle position), 
and mode (the value that occurs most often; 
a hash map will be helpful here) of the list.
*/

use std::collections::HashMap;

fn main() {
    // arbitrary list
    let v = vec![5, 4, 2, 1, 6, 1, 2];

    // arbitrary list but sorted
    let mut v_sorted = v.clone(); 
    v_sorted.sort(); // [1, 2, 2, 4, 5, 6]

    // mean
    let mut sum : f32 = 0.0;
    for val in &v{
        sum += *val as f32;
    }
    let _mean = sum/(v.len() as f32); // mean
    
    // median
    let mut _median : f32 = 0.0;
    if v.len() % 2 == 0 {
        _median = (v_sorted[&v_sorted.len()/2] as f32 + v_sorted[&v_sorted.len()/2 - 1] as f32)/2.0;
    }
    else{
        _median = (v_sorted[&v_sorted.len()/2] as f32)/2.0;

    }

    // mode
    let mut hm : HashMap<isize, isize> = HashMap::new();

    for val in &v{
        *hm.entry(*val).or_default() += 1;
    }

    let mut max = hm[&v[0]];
    let mut _mode = &v[0]; 
    // if two numbers occur the same ammount, 
    // get the one that comes first.
    
    for (key, val) in &hm{
        if max < *val{
            max = *val;
            _mode = key;
        }
    }

    println!("MEAN: {}, MEDIAN: {}, MODE: {}", _mean, _median, _mode);
}
