class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        hm1, hm2 = {}, {}

        if len(s) != len(t):
            return False

        for x in range(len(s)):
            if s[x] in hm1:
                hm1[s[x]] += 1
            else:
                hm1[s[x]] = 0
            if t[x] in hm2:
                hm2[t[x]] += 1
            else:
                hm2[t[x]] = 0

        return hm1 == hm2


# The overall time complexity is O(n), where n is the number of characters in the input string s/t. 
# 
# The process I went through here is that I knew I could use two hashmaps so I can traverse on of the strings once, 
# making the overall time complexity O(n). Here is the breakdown of my code:
#     1. I start my function by initializing both hashmaps as hm1 and hm2.
#     2. Then I return false if the length of the strings aren't equal.
#     3. Then I open a for-loop which indexes all number of characters there are in string s and t.
#     4. Then I return the equality of both hashmaps to check if the strings are valid anagrams.