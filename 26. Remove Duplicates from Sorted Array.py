class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        hm = {}
        out = []
        for x in range(len(nums)):
            if nums[x] not in hm:
                hm[nums[x]] = x
                out.append(nums[x])
        for x in range(len(nums)):
            if x >= len(out):
                nums[x] = None
            else:
                nums[x] = out[x]
        return len(out)


# The overall time complexity is O(n), where n is the number of elements in the input array. 
# 
# The process I went through here is knew I could use a hashmap to store the values and simply check if duplicates already
# existed by checking the hashmap, which ended up making the overall time complexity O(n). Here is the breakdown of my code:
#     1. I start my function by initializing my hashmap.
#     2. Then I initialize my temporary array called out.
#     3. Then I open a for-loop which indexes values in the hashmap if they're already not included (which eliminates duplicates)
#     4. Then I open another for-loop which replaces all elements in nums to the correct output, which is an ordered non-duplicate
#        array of integers with the end of the array typically being empty (which used to be the duplicates)
#     5. I then return the length of the out array (as per the question's demand) which is the length of the number of unique 
#        integers in the nums array. 