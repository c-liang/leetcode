//! # 49. 字母异位词分组
//!给定一个字符串数组，将字母异位词组合在一起。字母异位词指字母相同，但排列不同的字符串。

//! # 解题思路
//! 把字符长度，和，积 作为key值进行hash，一旦新字符跟hash值异或结果为0，则是一个字母异位

use std::collections::HashMap;
pub struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash_inputs: HashMap<(usize, u64, u64), Vec<Vec<String>>> = HashMap::new();
        for s in &strs {
            let mut sum = 0 as u64;
            let mut multiply = 1 as u64;
            let len = s.len();
            for b in s.as_bytes() {
                sum += (*b) as u64;
                //可能存在溢出
                multiply *= (*b) as u64;
            }
            //对原字符数组进行hash分类
            match hash_inputs.get_mut(&(len, sum, multiply)) {
                Some(v) => {
                    let mut insert_idx = v.len();
                    for i in 0..v.len() {
                        let mut xor = 0;
                        for j in 0..v[i][0].as_bytes().len() {
                            xor ^= v[i][0].as_bytes()[j];
                            xor ^= s.as_bytes()[j];
                        }
                        if xor == 0 {
                            insert_idx = i;
                            break;
                        }
                    }
                    //没有找到异位
                    if insert_idx == v.len() {
                        v.push(vec![s.clone(); 1]);
                    } else {
                        v[insert_idx].push(s.clone());
                    }
                }
                None => {
                    hash_inputs.insert((len, sum, multiply), vec![vec![s.clone(); 1]; 1]);
                }
            }
        }
        let mut ans = Vec::new();
        for v in hash_inputs {
            for d in v.1 {
                ans.push(d)
            }
        }
        ans
    }

    pub fn group_anagrams_new(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::new();
        strs.into_iter().for_each(|x| {
            let mut xx = [0; 26];
            x.bytes().for_each(|x| xx[(x - b'a') as usize] += 1);
            map.entry(xx).or_insert(vec![]).push(x)
        });
        let mut ans = Vec::new();
        for v in map {
            ans.push(v.1);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!(
            "{:?}",
            super::Solution::group_anagrams(["ids".into(), "tic".into()].into())
        );
    }
}
