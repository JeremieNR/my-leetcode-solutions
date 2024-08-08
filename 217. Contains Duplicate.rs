impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for x in 0..nums.len() {
            if (hm.contains_key(&nums[x])) {
                return true;
            } else {
                hm.insert(nums[x], 0 as i32);
            }
        }
        return false;
    }
}


// The overall time complexity is O(n), where n is the number of elements in the input vector nums. 
// 
// The process I went through here is that I knew I could use a hashmap so I can traverse the vector once, making the 
// overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing my hashmap as hm.
//     2. Then I open a for-loop which iterates through the elements of the nums input vector. In the loop, it returns
//        true if the element already exists in the hashmap, if not it adds it.
//     3. If the loop ends it returns false since there are no duplicate elements.