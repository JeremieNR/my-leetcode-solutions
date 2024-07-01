class Solution(object):
    def lengthOfLastWord(self, s):
        return len(s.split()[-1])


# The overall time complexity is O(n), where n is the number of characters in the input string. 
# 
# The process I went through here is knew I could simply strip the string and split it up, and take the length of the last word,
# which ended up making the overall time complexity O(n). The code is one line and does the following: The string is split into
# an array of strings, which also (in the process) removes white spaces. Then I simply return the length of the last string element
# from the array. 