use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let acc = nums.iter().enumerate().fold(
            HashMap::new(),
            |mut acc, (i, &n)| {
                acc.entry(&n).or_insert(vec![]).push(i as i32);
                acc
            },
        );
        
        let res = acc.clone().into_iter().find_map(
            |(num, idx)| match acc.get(&(target - num)) {
                Some(&idxs) => {
                    idxs.iter().find(|idx| if idx )
                },
                _ => None,
            },
        );

        match res {
            Some(v) => v,
            None => panic!("{:?}", acc),
        }
}
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_return_0_1() {
        assert_eq!(vec![0, 1], super::Solution::two_sum(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn should_also_return_0_1() {
        assert_eq!(vec![0, 1], super::Solution::two_sum(vec![3, 3], 6));
    }
}
