use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut results = HashSet::new();

    for x in 1u32..(sum/3) {
        for y in x+1u32..(sum/2) {
            let z = sum - x - y;
            if x.pow(2) + y.pow(2) == z.pow(2) {
                results.insert([x,y,z]);
            }
        }
    }
    results
}
