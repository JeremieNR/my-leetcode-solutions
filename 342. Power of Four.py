class Solution:
    def isPowerOfFour(self, n: int) -> bool:
        for x in range(16):
            if (4**x == n):
                return True
        return False


# The overall time complexity is O(1). 
# 
# The process I went through here is that I knew I could use the maximum integer (4^16) as a loop max, making the 
# overall time complexity O(1). I start my function by openning a for-loop, which checks if x is any power of 4. 
# If it is, it returns true, and if the for-loop ends, it means it isn't and thus returns false.
