class Solution:
    def reverse(self, x: int) -> int:
        sign = 1
        if x < 0:
            sign = -1
            x *= -1
        if (-2**31 < int(str(x)[::-1])*sign < 2**31 - 1):
            return int(str(x)[::-1])*sign
        return 0


# The overall time complexity is O(Log(x)), where x represents the input integer x.
# 
# The process I went through here is that I knew I could use built in python int() and str() functions, making the
# overall time complexity O(Log(n)). Here is the breakdown of my code:
#     1. I start my function by initializing the int sign as 1
#     2. I open an if statement to check if the input integer is negative, if it is, I set sign to -1 and I make
#        the input integer positive.
#     3. Then I open an if statement to check if the reverse integer is in the 32-bit integer range. If it is, it 
#        will return the reverse integer.
#     4. If the reverse integer is not in the 32-bit integer range, it will return zero by default. 