class Solution {
    public boolean containsDuplicate(int[] nums) {
        Map<Integer, Integer> hm = new HashMap<>();
        for (int x = 0; x < nums.length; x++) {
            if (hm.containsKey(nums[x])) {
                return true;
            } else {
                hm.put(nums[x], 0);
            }
        }
        return false;
    }
}


// The overall time complexity is O(n), where n is the number of elements in the input array nums. 
// 
// The process I went through here is that I knew I could use a hashmap so I can traverse the array once, making the 
// overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing my hashmap as hm.
//     2. Then I open a for-loop which iterates through the elements of the nums input array. In the loop, it returns
//        true if the element already exists in the hashmap, if not it adds it.
//     3. If the loop ends it returns false since there are no duplicate elements.