impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for x in 0..nums.len() {
            if (hm.contains_key(&(nums[x]))) {
                hm.insert(nums[x], hm[&nums[x]] + 1);
            } else {
                hm.insert(nums[x], 0);
            }
        }
        return *hm.iter().max_by_key(|x | x.1).unwrap().0;
    }
}


// The overall time complexity is O(n), where n is the number of elements in the input vector nums. 
// 
// The process I went through here is that I knew I could use a hashmap so I can traverse the vector once, making the 
// overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by getting the standard hashmap and initializing the mutable hashmap as hm.
//     2. Then I open a for-loop which iterates through the elements of the nums input vector. In the loop, it adds 1 to
//        the value if the key exists in the hashmap, or sets the given key to zero if it doesn't exist in the hashmap.
//     3. Then I return the key with the maximum value. 