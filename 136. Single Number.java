class Solution {
    public int singleNumber(int[] nums) {
        Map<Integer, Integer> hm = new HashMap<Integer, Integer>();
        for (int x = 0; x < nums.length; x++) {
            if (hm.containsKey(nums[x])) {
                hm.remove(nums[x]);
            } else {
                hm.put(nums[x], x);
            }
        }
        return hm.keySet().iterator().next();
    }
}


// The overall time complexity is O(n), where n represents the number of elements in the input array.
// 
// The process I went through here is that I knew I could use a hashmap to store integers and check
// for duplicates, making the overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by getting the standard hashmap and initializing the mutable hashmap as hm.
//     2. Then I open a for-loop and use an if-else statement to see if the key already exists 
//        in my hashmap. If it does, it removes it, and if it doesn't, it adds it.
//     3. Then I return the only key stored in the hashmap.