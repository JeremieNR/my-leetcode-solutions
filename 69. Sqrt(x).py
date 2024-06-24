class Solution:
    def mySqrt(self, x: int) -> int:
        i = x
        while abs(i*i - x) > 0.75:
            i = (i+(x/i))/2
        return math.floor(i)


# The overall time complexity is O(Log(n)), where n represents the input integer x.
# 
# The process I went through here is that I knew I could use Newton's method to approximate the square root of x, making
# the overall time complexity O(Log(n)). Here is the breakdown of my code:
#     1. I start my function by initializing the integer i equal to the target value x.
#     2. Then I open a while-loop which keeps on iterating through Newton's method until our estimate i is a close enough
#        estimate for the square root of x (within 0.75).
#     3. Then I simply return the floored integer from the estimate i.