public class Solution {
    public int[] SingleNumber(int[] nums) {
        IDictionary<int, int> hm = new Dictionary<int, int>();
        List<int> outList = new List<int>();
        for (int x = 0; x < nums.Length; x++) {
            if (hm.ContainsKey(nums[x]) == false) {
                hm[nums[x]] = 0;
            } else {
                hm.Remove(nums[x]);
            }
        }
        foreach (var x in hm) {
            outList.Add(x.Key);
        }
        return outList.ToArray();
    }
}


// The overall time complexity is O(n), where n represents the number of elements in the input array nums.
// 
// The process I went through here is that I knew I could a hashmap to keep track of single numbers, making the
// overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing the hashmap hm and the temporary list outList.
//     2. Then I open a for-loop which adds a number in the hashmap if it isn't already in it. If it is, it deletes it.
//     3. Then I add all the keys from the hashmap hm in the list outList.
//     4. Then I return the array of keys from the hashmap hm.