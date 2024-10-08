class Solution {
    public int findClosestNumber(int[] nums) {
        int[] pair = new int[] {};
        int minimum = 100001;
        for (int x = 0; x < nums.length; x++) {
            if (Math.abs(nums[x]) < minimum) {
                minimum = Math.abs(nums[x]);
                pair = new int[] {nums[x], x};
            } else if ((Math.abs(nums[x]) == minimum) && (nums[x] > pair[0])) {
                pair = new int[] {nums[x], x};
            }
        }
        return pair[0];
    }
}


// The overall time complexity is O(n), where n is the number of elements in the input array. 
// 
// The process I went through here is that I knew I could use a list as a value-index pair for 
// the largest number with the smallest distance to zero, making the overall time complexity O(n).
// Here is the breakdown of my code:
//     1. I start my function by initializing the pair list and minimum integer.
//     2. Then, I open a for-loop to iterate through all the indexes of the array. If a new minimum 
//        is found, the minimum integer is updated and so is the pair list. If not, if the distance 
//        to zero is equal to the current minimum and the integer is larger than the current pair value,
//        it will be updated.
//     3. After the for-loop, the largest integer with the smallest distance to zero is returned.
