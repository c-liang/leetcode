pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    if s.len() < 3 {
        return vec![];
    }
    let bytes = s.as_bytes();
    let mut ans = vec![];
    ans.push(vec![0, 1]);
    let mut pre_c = bytes[0];
    let mut start = 0;
    let mut end = 0;
    for i in 1..bytes.len() {
        if bytes[i] == pre_c {
            end = i;
        } else {
            if (end - start + 1) >= 3 {
                ans.push(vec![start as i32, end as i32]);
            }
            start = i;
            end = i;
            pre_c = bytes[i];
        }
    }
    if (end - start + 1) >= 3 {
        ans.push(vec![start as i32, end as i32]);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        split_array_into_fibonacci_sequence::Solution::split_into_fibonacci("0123".into())
    );
    println!("Hello, world!");
}
