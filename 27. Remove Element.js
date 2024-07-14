var removeElement = function(nums, val) {
    let out = [];
    for (let x = 0; x < nums.length; x++) {
        if (nums[x] != val) {
            out.push(nums[x]);
        }
    }
    for (let x = 0; x < nums.length; x++) {
        if (x < out.length) {
            nums[x] = out[x];
        }
    }
    return out.length;
};


// The overall time complexity is O(n), where n is the number of elements in the input array. 
// 
// The process I went through here is knew I could simply add the filtered elements to another array and later add it back to the original,
// which ended up making the overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing my temporary array called out.
//     2. Then I open a for-loop which indexes values in the temporary array out if they're not the val integer.
//     3. Then I open another for-loop which replaces all elements in nums to the correct output, which is an ordered non-duplicate
//        array of integers with the end of the array typically being artifact integers from the original array.
//     4. I then return the length of the out array (as per the question's demand) which is the length of the non-val integers in the 
//        nums array. 