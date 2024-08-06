var majorityElement = function(nums) {
    let hm = {};
    for (let i = 0; i < nums.length; i++) {
        if (hm[nums[i]] !== undefined) {
            hm[nums[i]] = hm[nums[i]] + 1;
        } else {
            hm[nums[i]] = 0;
        }
    }
    return Object.keys(hm).reduce((a, b) => hm[a] > hm[b] ? a : b);
};


// The overall time complexity is O(n), where n is the number of elements in the input array nums. 
// 
// The process I went through here is that I knew I could use a hashmap so I can traverse the array once, making the 
// overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing my hashmap as hm.
//     2. Then I open a for-loop which iterates through the elements of the nums input array. In the loop, it adds 1 to
//        the value if the key exists in the hashmap, or sets the given key to zero if it doesn't exist in the hashmap.
//     3. Then I return the key with the maximum value. 