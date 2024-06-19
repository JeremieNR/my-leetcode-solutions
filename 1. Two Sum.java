class Solution {
    public int[] twoSum(int[] nums, int target) {
        int[] out = {0, 0};
        Map<Integer, Integer> hm = new HashMap<Integer, Integer>();
        for (int x = 0; x < nums.length; x++) {
            if (hm.containsKey(target - nums[x])) {
                out[0] = x;
                out[1] = hm.get(target - nums[x]);
                return out;
            } else {
                hm.put(nums[x], x);
            }
        }
        return out;
    }
}


// The overall time complexity is O(n), where n is the number of elements in the input array. 
// 
// The process I went through here is that I knew I could use a hashmap so I can traverse the array once, making the 
// overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing the output integer array and setting it to {0, 0}.
//     2. Then, I initializing the hashmap as hm.
//     3. Then, I open a for-loop to iterate through all the indexes of the array. 
//     4. Then, I add an if-else statement to have two cases:
//         a) if the compliment* value exists in the hashmap, update the integer array to add the two indexes
//            whose values add up to the target and return it.
//         b) if the compliment* value does not exist in the hashmap, add it to the hashmap as a value-index pair. 
//     5. It will keep iterating the array until the solution pair is returned since there will always be a solution. 
//     6. For the sake of not getting a "not all code paths return a value" error, I return the out integer array
//        after the for-loop.
//
//
// *compliment value is equivalent to target - nums[x]. 