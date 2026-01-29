class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        output = ""
        minimum = len(min(strs, key=len))

        for i in range(minimum):
            char = strs[0][i]
            for word in strs:
                if word[i] != char:
                    return output
            output += char

        return output


# The overall time complexity is O(n*m), where n represents the number of strings and m represents the length of the shortest string.
# 
# The process I went through here is that I knew I could compare characters across all strings, up to the shortest string's length. Here is the breakdown of my code:
#     1. Find the length of the shortest string.
#     2. For each index up to that length, take the character from the first string.
#     3. Compare that character with the character at the same index in every string; append if all match, otherwise return.
