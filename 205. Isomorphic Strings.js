var isIsomorphic = function(s, t) {
    let hm1 = {};
    let hm2 = {};
    for (let x = 0; x < s.length; x++) {
        if (((hm1[t[x]] != undefined) && (s[x] != hm1[t[x]])) || ((hm2[s[x]] != undefined) && (t[x] != hm2[s[x]]))) {
            return false;
        } else {
            hm2[s[x]] = t[x];
            hm1[t[x]] = s[x];
        }
    }
    return true;
};


// The overall time complexity is O(n), where n represents the length of the input strings.
// 
// The process I went through here is that I knew I could use a hashmap to store character data from one string to the other, 
// making the overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing both hashmaps.
//     2. Then I open a for-loop which checks if a character comparison is wrong in either hashmaps. If so, returns False. The
//        for-loop also indexes characters if they don't exist in the hashmaps.
//     3. If the for-loop ends, True is returned.