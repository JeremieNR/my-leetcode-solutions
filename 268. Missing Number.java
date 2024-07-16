class Solution {
    public int missingNumber(int[] nums) {
        int maximum = 0;
        Map<Integer, Integer> hm = new HashMap<Integer, Integer>();
        for (int x = 0; x < nums.length; x++) {
            hm.put(nums[x], 0);
            if (nums[x] > maximum) {
                maximum = nums[x];
            }
        }
        for (int x = 0; x < maximum; x++) {
            if (!hm.containsKey(x)) {
                return x;
            }
        }
        return maximum + 1;
    }
}


// The overall time complexity is O(n), where n is the number of elements in the input array. 
//
// The process I went though here is that I knew I could use a hashmap so I can traverse the array once, making the
// overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing the maximum int and the hashmap as hm. 
//     2. Then I open a for-loop to index the elements of the array in the hashmap, and update the maximum int in 
//        the array.
//     3. Then I open another for-loop to check which element in the "full" array does not exist in the indexed array
//        that is in the hashmap. If it finds it, it returns it.
//     4. If the loop ends, then the integer is the maximum + 1.