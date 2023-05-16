use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let acc = nums
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, &n)| {
                acc.entry(n).or_insert(vec![]).push(i);
                acc
            });

        for (&num, idxs) in &acc {
            let diff = target - num;
            let &idx = idxs.first().unwrap();

            let res = match acc.get(&diff) {
                Some(other_idxs) => other_idxs.iter().find(|&&x| x != idx),
                None => continue,
            };

            match res {
                Some(&v) => return vec![idx as i32, v as i32],
                None => continue,
            }
        }
        panic!("No solution found")
    }
}

#[cfg(test)]
mod tests {
    fn check(nums: &Vec<i32>, target: i32) {
        let res = super::Solution::two_sum(nums.to_owned(), target);
        assert!(res.len() == 2);
        assert!(res[0] != res[1]);
        assert!(nums[res[0] as usize] + nums[res[1] as usize] == target);
    }
    #[test]
    fn simple() {
        check(&vec![2, 7, 11, 15], 9);
    }

    #[test]
    fn should_also_return_0_1() {
        check(&vec![3, 3], 6);
    }
}
