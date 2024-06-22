impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut out = vec![];
        for x in 0..nums.len() {
            if (nums[x] != val) {
                out.push(nums[x]);
            }
        }
        for x in 0..nums.len() {
            if (x < out.len()) {
                nums[x] = out[x];
            }
        }
        return out.len().try_into().unwrap();
    }
}


// The overall time complexity is O(n), where n is the number of elements in the input vector. 
// 
// The process I went through here is knew I could simply add the filtered elements to another vector and later add it back to the original,
// which ended up making the overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing my temporary vector called out.
//     2. Then I open a for-loop which indexes values in the temporary vector out if they're not the val integer.
//     3. Then I open another for-loop which replaces all elements in nums to the correct output, which is an ordered non-duplicate
//        vector of integers with the end of the vector typically being artifact integers from the original vector.
//     4. I then return the length of the out vector (as per the question's demand) which is the length of the non-val integers in the 
//        nums vector. 