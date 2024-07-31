impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut pair : Vec<i32> = Vec::new();
        let mut minimum : i32 = 100001;
        for x in 0..nums.len() {
            if (nums[x].abs() < minimum) {
                minimum = nums[x].abs();
                pair = vec![nums[x], x as i32];
            } else if ((nums[x].abs() == minimum) && (nums[x] > pair[0])) {
                pair = vec![nums[x], x as i32];
            }
        }
        return pair[0];
    }
}


// The overall time complexity is O(n), where n is the number of elements in the input vector. 
// 
// The process I went through here is that I knew I could use a vector as a value-index pair for 
// the largest number with the smallest distance to zero, making the overall time complexity O(n).
// Here is the breakdown of my code:
//     1. I start my function by initializing the pair vector and minimum integer.
//     2. Then, I open a for-loop to iterate through all the indexes of the vector. If a new minimum 
//        is found, the minimum integer is updated and so is the pair vector. If not, if the distance 
//        to zero is equal to the current minimum and the integer is larger than the current pair value,
//        it will be updated.
//     3. After the for-loop, the largest integer with the smallest distance to zero is returned.