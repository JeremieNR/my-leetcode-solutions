public class Solution {
    public int RemoveElement(int[] nums, int val) {
        List<int> outList = new List<int>();
        for (int x = 0; x < nums.Length; x++) {
            if (nums[x] != val) {
                outList.Add(nums[x]);
            }
        }
        for (int x = 0; x < nums.Length; x++) {
            if (x < outList.Count) {
                nums[x] = outList[x];
            }
        }
        return outList.Count;
    }
}


// The overall time complexity is O(n), where n is the number of elements in the input array. 
// 
// The process I went through here is knew I could simply add the filtered elements to another array and later add it back to the original,
// which ended up making the overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing my temporary array called outList.
//     2. Then I open a for-loop which indexes values in the temporary array out if they're not the val integer.
//     3. Then I open another for-loop which replaces all elements in nums to the correct output, which is an ordered non-duplicate
//        array of integers with the end of the array typically being artifact integers from the original array.
//     4. I then return the length of the outList array (as per the question's demand) which is the length of the non-val integers in the 
//        nums array. 
