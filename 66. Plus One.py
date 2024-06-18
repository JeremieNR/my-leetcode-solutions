class Solution:
    def plusOne(self, digits: List[int]) -> List[int]:
        val = 0
        for x in range(len(digits) - 1, -1, -1):
            val += (10 ** x) * digits[len(digits) - x - 1]
        return map(int, str(val + 1))


# The overall time complexity is O(n), where n is the number of elements in the input array. 
# 
# The process I went through here is that I knew I should use powers of 10 to convert the array to an int, where I would
# traverse the array once making the overall time complexity O(n). Here is the breakdown of my code:
#     1. I start my function by initializing the integer val.
#     2. Then, I open a for-loop to iterate through all the indexes of the array, starting from the highest, and going
#        down to index zero.
#     3. Then, I add each power of 10 times its leading digit to val.
#     4. Finally I use a mapping to convert val + 1 from an integer to an array.