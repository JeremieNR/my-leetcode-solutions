impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut hm: HashMap<i32, i32> = HashMap::new();
        let mut out = vec![];
        for x in 0..nums.len() {
            if !(hm.contains_key(&nums[x])) {
                hm.insert(nums[x], x as i32);
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
// The process I went through here is knew I could use a hashmap to store the values and simply check if duplicates already
// existed by checking the hashmap, which ended up making the overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by getting the standard hashmap and initializing the mutable hashmap as hm.
//     2. Then I initialize my temporary vector called out.
//     3. Then I open a for-loop which indexes values in the hashmap if they're already not included (which eliminates duplicates)
//     4. Then I open another for-loop which replaces all elements in nums to the correct output, which is an ordered non-duplicate
//        vector of integers with the end of the vector typically being random integers which were artifacts from the original vector.
//     5. I then return the length of the out vector (as per the question's demand) which is the length of the number of unique 
//        integers in the nums vector. 