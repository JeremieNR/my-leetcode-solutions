var solution = function(isBadVersion) {
    return function(n) {
        low = 0;
        mid = n/2;
        while ((low != mid) && (mid != n)) {
            if (isBadVersion(mid) == false) {
                low = mid;
            } else {
                n = mid;
            }
            mid = (low + n)/2;
        }
        return mid;
    };
};


// The overall time complexity is O(Log(n)), where n represents the input integer n.
// 
// The process I went through here is that I knew I could use a binary search to find the smallest bad version,
// making the overall time complexity O(Log(n)). Here is the breakdown of my code:
//     1. I start my function by initializing low and mid. 
//     2. Then I open a while-loop which will keep dividing the search area until it finds the smallest bad version.
//     3. Then I return mid + 1 (which is the smallest bad version).
