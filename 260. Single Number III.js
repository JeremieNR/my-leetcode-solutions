var singleNumber = function(nums) {
    let hm = {};
    for (let x = 0; x < nums.length; x++) {
        if (hm[nums[x]] == undefined) {
            hm[nums[x]] = 0;
        } else {
            delete hm[nums[x]];
        }
    }
    return Object.keys(hm);
};


// The overall time complexity is O(n), where n represents the number of elements in the input array nums.
// 
// The process I went through here is that I knew I could a hashmap to keep track of single numbers, making the
// overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing the hashmap hm.
//     2. Then I open a for-loop which adds a number in the hashmap if it isn't already in it. If it is, it deletes it.
//     3. Then I return an array of keys from the hashmap hm.