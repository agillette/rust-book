use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 3, 12, 2, 6, 7, 3, 9, 33, 5];

    v.sort_by(|a, b| a.cmp(b));
    
    let mut length = 0;

    let mut total = 0;
    
    for i in &v {
        total += i;
        length += 1;
        println!("total is: {}", total);
        println!("length is: {}", length);
    }

    let mean = &total / &length;
    println!("mean is: {}", mean);

    let median = &v[&length / 2];
    println!("median is: {}", median);

    // use this map to discover mode
    let mut map = HashMap::new();

    for i in &v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
