class Solution:
    def isPalindrome(self, s: str) -> bool:
        temp = ""
        for x in s:
            if x.isalnum():
                temp += x.lower()
        return temp == temp[::-1]


# The overall time complexity is O(n), where n represents the number of characters in the input string s.
# 
# The process I went through here is that I knew I could save alphanumeric characters in a temporary string and 
# compare that string to its reverse, making the overall time complexity O(n). Here is the breakdown of my code:
#     1. I start my function by initializing the temporary string temp.
#     2. Then I open a for-loop to check if a character is alphanumeric, and if it is I save it's lowercase to 
#        the temp string.
#     3. Then I return the boolean which represents the equality of comparing the temp string to its reverse.