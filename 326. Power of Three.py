class Solution:
    def isPowerOfThree(self, n: int) -> bool:
        for exp in range(20):
            if 3**exp == n:
                return True
        return False


# The overall time complexity is O(1). 
# 
# The process I went through here is that I knew I could use the maximum integer (3^20) as a loop max, making the 
# overall time complexity O(1). I start my function by openning a for-loop, which checks if x is any power of 3. 
# If it is, it returns true, and if the for-loop ends, it means it isn't and thus returns false.