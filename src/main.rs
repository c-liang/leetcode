pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    if intervals.is_empty() {
        return 0;
    }
    //动态规划
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut dp = vec![1; intervals.len()];
    for i in 1..intervals.len() {
        for j in 0..=i {
            if intervals[j][1] <= intervals[i][0] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    let mut max = 0;
    for v in dp {
        if v > max {
            max = v;
        }
    }
    (intervals.len() - max) as i32
}
pub fn erase_overlap_intervals_(mut intervals: Vec<Vec<i32>>) -> i32 {
    if intervals.is_empty() {
        return 0;
    }
    intervals.sort_by(|a, b| a[1].cmp(&b[1]));
    let mut ans = 1;
    let mut right = intervals[0][1];
    for i in 1..intervals.len() {
        if intervals[i][0] >= right {
            ans += 1;
            right = intervals[i][1];
        }
    }
    intervals.len() as i32 - ans
}

fn main() {
    println!(
        "{:?}",
        split_array_into_fibonacci_sequence::Solution::split_into_fibonacci("0123".into())
    );
    println!("Hello, world!");
}
