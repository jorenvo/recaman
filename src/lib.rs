use std::collections::HashSet;

pub fn recaman_sequence(n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![];
    }

    let mut used_numbers = HashSet::new();
    let mut seq: Vec<u64> = vec![0];
    while seq.len() < n as usize {
        let seq_len = seq.len() as i64;
        let last = *seq.last().unwrap() as i64;
        let subtract_result = last - seq_len;

        let a: i64;
        if subtract_result > 0 && !used_numbers.contains(&(subtract_result as u64)) {
            a = subtract_result;
        } else {
            a = last + seq_len;
        }

        seq.push(a as u64);
        used_numbers.insert(a as u64);
    }

    seq
}
