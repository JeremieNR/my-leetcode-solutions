var singleNumber = function(nums) {
    let hm = new Map();
    for (let x = 0; x < nums.length; x++) {
        hm.has(nums[x]) ? hm.delete(nums[x]):hm.set(nums[x], x);
    }
    return hm.keys().next().value;
};


// The overall time complexity is O(n), where n represents the number of elements in the input array.
// 
// The process I went through here is that I knew I could use a hashmap to store integers and check
// for duplicates, making the overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing the hashmap as hm.
//     2. Then I open a for-loop and use a short hand if-else statement to see if the key already exists 
//        in my hashmap. If it does, it removes it, and if it doesn't, it adds it.
//     3. Then I return the only key stored in the hashmap.