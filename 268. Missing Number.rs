impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut hm: HashMap<i32, i32> = HashMap::new();
        let mut maximum = 0;
        for x in 0..nums.len() {
            hm.insert(nums[x], 0);
            if (nums[x] > maximum) {
                maximum = nums[x];
            }
        }
        for x in 0..maximum {
            if (!hm.contains_key(&x)) {
                return x;
            }
        }
        return maximum + 1;
    }
}


// The overall time complexity is O(n), where n is the number of elements in the input vector. 
//
// The process I went though here is that I knew I could use a hashmap so I can traverse the vector once, making the
// overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing the maximum int and the hashmap as hm. 
//     2. Then I open a for-loop to index the elements of the vector in the hashmap, and update the maximum int in 
//        the vector.
//     3. Then I open another for-loop to check which element in the "full" vector does not exist in the indexed vector
//        that is in the hashmap. If it finds it, it returns it.
//     4. If the loop ends, then the integer is the maximum + 1.
