class Solution:
    def isPalindrome(self, x: int) -> bool:
        return (str(x)[::-1] == str(x))


# The overall time complexity is O(n), where n is the number of characters in the string representing x. 
# 
# The process I went through here is that I knew I could convert the integer to a string, and compare itself with its
# reversed string, making the overall time complexity O(n). How I acheive this is by using str(x), which converts the 
# integer into a string, and then using str(x)[::-1] to reverse the string, I return the comparison between the reversed
# string and the original string. This returns a bool saying if the reverse is equal to the original. 
