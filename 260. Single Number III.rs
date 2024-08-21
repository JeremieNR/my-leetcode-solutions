impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for x in 0..nums.len() {
            if (!hm.contains_key(&nums[x])) {
                hm.insert(nums[x], 0);
            } else {
                hm.remove(&nums[x]);
            }
        }
        return hm.keys().cloned().collect();
    }
}


// The overall time complexity is O(n), where n represents the number of elements in the input vector nums.
// 
// The process I went through here is that I knew I could a hashmap to keep track of single numbers, making the
// overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by getting the standard hashmap and initializing the hashmap hm.
//     2. Then I open a for-loop which adds a number in the hashmap if it isn't already in it. If it is, it deletes it.
//     3. Then I return a vector of keys from the hashmap hm.