class Solution:
    def singleNumber(self, nums: List[int]) -> List[int]:
        hm = {}
        for x in nums:
            if x not in hm:
                hm[x] = 0
            else:
                del hm[x]
        return hm.keys()


# The overall time complexity is O(n), where n represents the number of elements in the input array nums.
# 
# The process I went through here is that I knew I could a hashmap to keep track of single numbers, making the
# overall time complexity O(n). Here is the breakdown of my code:
#     1. I start my function by initializing the hashmap hm.
#     2. Then I open a for-loop which adds a number in the hashmap if it isn't already in it. If it is, it deletes it.
#     3. Then I return an array of keys from the hashmap hm.