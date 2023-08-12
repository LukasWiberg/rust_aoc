use std::fs;

pub(crate) fn task() {
    let file_content = fs::read_to_string("input_1.txt").unwrap();
    let sets = file_content.split("\n\n");
    let set_count = sets.clone().count();
    let mut sets_sum = vec![0; set_count];
    let mut i = 0;
    for set in sets {
        let vals = set.split("\n");
        let mut sum: i32 = 0;
        for val in vals {
            if val.len() == 0 {
                continue;
            }
            sum += val.parse::<i32>().unwrap();
        }
        sets_sum[i] = sum;
        i += 1;
    }
    sets_sum.sort();
    println!("ayo: {}", sets_sum[set_count-1] + sets_sum[set_count-2] + sets_sum[set_count-3]);

