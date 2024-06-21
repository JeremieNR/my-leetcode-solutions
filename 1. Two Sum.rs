impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for x in 0..nums.len() {
            if (hm.contains_key(&(target - nums[x]))) {
                return vec![*(hm.get(&(target - nums[x]))).unwrap_or(&0), x as i32];
            } else {
                hm.insert(nums[x], x as i32);
            }
        }
        return vec![];
    }
}


// The overall time complexity is O(n), where n is the number of elements in the input vector. 
// 
// The process I went through here is that I knew I could use a hashmap so I can traverse the vector once, making the 
// overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by getting the standard hashmap and initializing the mutable hashmap as hm.
//     2. Then, I open a for-loop to iterate through all the indexes of the vector. 
//     3. Then, I add an if-else statement to have two cases:
//         a) if the compliment* value exists in the hashmap, return the two indexes whose values add up to the target.
//         b) if the compliment* value does not exist in the hashmap, add it to the hashmap as a value-index pair. 
//     4. It will keep iterating the vector until the solution pair is returned since there will always be a solution. 
//     5. For the sake of not getting a "not all code paths return a value" error, I return an empty vector after the
//        for-loop.
//
//
// *compliment value is equivalent to target - nums[x]. 