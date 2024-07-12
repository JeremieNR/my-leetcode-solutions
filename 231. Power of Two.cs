public class Solution {
    public bool IsPowerOfTwo(int n) {
        for (int x = 0; x < 31; x++) {
            if (Math.Pow(2, x) == n) {
                return true;
            }
        }
        return false;
    }
}


// The overall time complexity is O(1). 
// 
// The process I went through here is that I knew I could use the maximum integer (2^32) as a loop max, making the 
// overall time complexity O(1). I start my function by openning a for-loop, which checks if n is any power of 2. 
// If it is, it returns true, and if the for-loop ends, it means it isn't and thus returns false.