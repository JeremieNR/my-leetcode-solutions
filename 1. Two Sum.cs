public class Solution {
    public int[] TwoSum(int[] nums, int target) {
        IDictionary<int, int> hm = new Dictionary<int, int>();
        for (int x = 0; x < nums.Length; x++) {
            if (hm.ContainsKey(target - nums[x])) {
                return [x, hm[target - nums[x]]];
            } else {
                hm[nums[x]] = x;
            }
        }
        return [];
    }
}


// The overall time complexity is O(n), where n is the number of elements in the input array. 
// 
// The process I went through here is that I knew I could use a hashmap so I can traverse the array once, making the 
// overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing the hashmap as hm. In this case, it's a dictionary type. 
//     2. Then, I open a for-loop to iterate through all the indexes of the array. 
//     3. Then, I add an if-else statement to have two cases:
//         a) if the compliment* value exists in the hashmap, return the two indexes whose values add up to the target.
//         b) if the compliment* value does not exist in the hashmap, add it to the hashmap as a value-index pair. 
//     4. It will keep iterating the array until the solution pair is returned since there will always be a solution. 
//     5. For the sake of not getting a "not all code paths return a value" error, I return an empty array after the
//        for-loop.
//
//
// *compliment value is equivalent to target - nums[x]. 