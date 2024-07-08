var intersection = function(nums1, nums2) {
    let out = [];
    let hm = new Map();
    for (let x = 0; x < nums1.length; x++) {
        if (hm[nums1[x]] == undefined) {
            hm[nums1[x]] = 0;
        }
    }
    for (let x = 0; x < nums2.length; x++) {
        if ((hm[nums2[x]] !== undefined) && (!out.includes(nums2[x]))) {
            out.push(nums2[x]);
        }
    }
    return out;
};


// The overall time complexity is O(n + m), where n is the length of nums1 and m is the length of nums2.
// 
// The process I went through here is that I knew I could use a hashmap so I can traverse one of the arrays once, making the 
// overall time complexity O(n + m). Here is the breakdown of my code:
//     1. I start my function by initializing the output array out, and the hashmap hm.
//     2. Then I open a for-loop that indexes the integers from the nums1 array just once (in the hashmap).
//     3. Then I open another for-loop that adds the integers who are in the hashmap and not in the out array to the out array.
//     4. Then I return the out array which contains the intersection integers from nums1 and nums2.
