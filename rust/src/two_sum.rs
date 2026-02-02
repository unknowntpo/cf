use std::collections::HashMap;

/// LeetCode 1. Two Sum
///
/// Given an array of integers `nums` and an integer `target`,
/// return indices of the two numbers such that they add up to `target`.
///
/// Assumptions:
/// - Each input has exactly one solution
/// - You may not use the same element twice
/// - Answer can be returned in any order
///
/// Example:
/// Input: nums = [2,7,11,15], target = 9
/// Output: [0,1] (because nums[0] + nums[1] == 9)

// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
// 	for i in 0..nums.len() {
//         let a = nums[i];
//         for j in i+1.. nums.len() {
//             let b = nums[j];
//             if a + b == target {
//                 return vec![i as i32, j as i32];
//             }
//         }
// 	};
//     return vec![];
// }

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // valA + valB = target
    // m[target-valA] = idxA
    // m[nums[idxB]] exist:  -> return vec[[nums[idxB], idxB],
    // map: target - valB
    let mut m: HashMap<i32, i32> = HashMap::new();

    for (i, &v) in nums.iter().enumerate() {
        if let Some(&j) = m.get(&v) {
            return vec![j, i as i32];
        }
        m.insert(target - v, i as i32);
    }

    println!("{:?}", m);

    vec![]
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_basic() {
		let result = two_sum(vec![2, 7, 11, 15], 9);
		assert_eq!(result, vec![0, 1]);
	}

	#[test]
	fn test_not_adjacent() {
		let result = two_sum(vec![3, 2, 4], 6);
		assert_eq!(result, vec![1, 2]);
	}

	#[test]
	fn test_same_value() {
		let result = two_sum(vec![3, 3], 6);
		assert_eq!(result, vec![0, 1]);
	}

	#[test]
	fn test_negative_numbers() {
		let result = two_sum(vec![-1, -2, -3, -4, -5], -8);
		assert_eq!(result, vec![2, 4]);
	}

	#[test]
	fn test_mixed_signs() {
		let result = two_sum(vec![-3, 4, 3, 90], 0);
		assert_eq!(result, vec![0, 2]);
	}
}
