struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        
        let mut k = 0;
        
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        
        k as i32
    }
}

fn main() {
    // Example usage
    let mut nums1 = vec![3, 2, 2, 3];
    let val1 = 3;
    let k1 = Solution::remove_element(&mut nums1, val1);
    println!("Example 1: k = {}, nums = {:?}", k1, &nums1[0..k1 as usize]);
    
    let mut nums2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val2 = 2;
    let k2 = Solution::remove_element(&mut nums2, val2);
    println!("Example 2: k = {}, nums = {:?}", k2, &nums2[0..k2 as usize]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut nums1 = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut nums1, 3), 2);
        
        let mut nums2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut nums2, 2), 5);
        
        let mut nums3 = vec![];
        assert_eq!(Solution::remove_element(&mut nums3, 1), 0);
    }
}