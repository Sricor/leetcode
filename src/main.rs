// https://leetcode.cn/problems/two-sum/
// 1, 0 ms, 2.50 MB
use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (index, value) in nums.iter().enumerate() {
        let key = target - *value;
        if let Some(index2) = map.get(&key) {
            return vec![*index2 as i32, index as i32];
        }
        map.insert(*value, index);
    }

    panic!()
}

// https://leetcode.cn/problems/reverse-integer/
// 7, 0 ms, 1.91 MB
pub fn reverse(x: i32) -> i32 {
    x.abs()
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
        .unwrap_or(0)
        * x.signum()
}

// https://leetcode.cn/problems/palindrome-number/
// 9, 0 ms, 1.92 MB
pub fn is_palindrome(x: i32) -> bool {
    let mut bytes = x.to_string().into_bytes();
    let vec = bytes.clone();
    bytes.reverse();

    if vec == bytes {
        true
    } else {
        false
    }
}

// https://leetcode.cn/problems/3sum
// 15, 32 ms, 3.86 MB
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = Vec::new();
    nums.sort_unstable();
    for i in 0..nums.len() {
        if nums[i] > 0 {
            break;
        }
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let (mut l, mut r) = (i + 1, nums.len() - 1);
        while l < r {
            let sum = nums[i] + nums[l] + nums[r];
            if sum == 0 {
                ret.push(vec![nums[i], nums[l], nums[r]]);
                while r < nums.len() - 1 && nums[r] == nums[r - 1] {
                    r -= 1;
                }
                while l < nums.len() - 1 && nums[l] == nums[l + 1] {
                    l += 1;
                }
            }
            if sum > 0 {
                r -= 1;
            } else {
                l += 1;
            }
        }
    }
    ret
}

// https://leetcode.cn/problems/remove-duplicates-from-sorted-array/
// 26, 0 ms, 2.20 MB
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut lhs = 0;
    let mut rhs = 1;
    loop {
        if rhs >= nums.len() {
            break;
        }
        if nums[rhs] != nums[rhs - 1] {
            lhs += 1;
            nums[lhs] = nums[rhs];
        }
        rhs += 1;
    }
    (lhs + 1) as i32
}

// https://leetcode.cn/problems/merge-sorted-array/
// 88, 0 ms, 1.93 MB
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    nums1.truncate(m as usize);
    nums2.truncate(n as usize);
    nums1.append(nums2);
    nums1.sort();
}

// https://leetcode.cn/problems/pascals-triangle/
// 118, 0 ms, 2.04 MB
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();
    if num_rows == 0 {
        return ans;
    }
    ans.push(vec![1]);
    for index in 1..num_rows {
        let mut array = Vec::new();
        array.resize(index as usize + 1, 1 as i32);
        array[index as usize] = 1;
        for j in 1..index {
            array[j as usize] =
                ans[index as usize - 1][j as usize - 1] + ans[index as usize - 1][j as usize];
        }
        ans.push(array);
    }
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(120), 21);
        assert_eq!(reverse(0), 0);
        assert_eq!(reverse(-1534236469), 0);
        assert_eq!(reverse(1534236469), 0);
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
    }

    #[test]
    fn test_three_sum() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![[-1, -1, 2], [-1, 0, 1]]
        );
        // assert_eq!(three_sum(vec![0, 1, 1]), vec![vec![]]);
        assert_eq!(three_sum(vec![0, 0, 0]), vec![[0, 0, 0]]);
    }

    #[test]
    fn test_remove_duplicates() {
        let mut vec1 = vec![1, 1, 2];
        assert_eq!(remove_duplicates(&mut vec1), 2);
        assert_eq!(vec1, vec![1, 2, 2]);

        let mut vec1 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut vec1), 5);
        assert_eq!(vec1, vec![0, 1, 2, 3, 4, 2, 2, 3, 3, 4]);
    }

    #[test]
    fn test_merge() {
        let mut vec1 = vec![1, 2, 3, 0, 0, 0];
        let mut vec2 = vec![2, 5, 6];
        merge(&mut vec1, 3, &mut vec2, 3);
        assert_eq!(vec1, vec![1, 2, 2, 3, 5, 6]);

        let mut vec1 = vec![1];
        let mut vec2 = vec![];
        merge(&mut vec1, 1, &mut vec2, 0);
        assert_eq!(vec1, vec![1]);

        let mut vec1 = vec![0];
        let mut vec2 = vec![1];
        merge(&mut vec1, 0, &mut vec2, 1);
        assert_eq!(vec1, vec![1]);
    }

    #[test]
    fn test_generate() {
        assert_eq!(generate(2), vec![vec![1], vec![1, 1]]);
        assert_eq!(
            generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}

fn main() {
    println!("Hello, world!");
}
