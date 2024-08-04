class Solution {
    public int hammingWeight(int n) {
        int bits = 0;
        int size = 32;
        while (n != 0) {
            if (n >= Math.pow(2, size)) {
                n -= (int)Math.pow(2, size);
                bits += 1;
            }
            size -= 1;
        }
        return bits;
    }
}


// The overall time complexity is O(Log(n)), where n represents the input integer n.
// 
// The process I went through here is that I knew I could use exponents to check each power of 2 for 
// the integer, making the overall time complexity O(Log(n)). Here is the breakdown of my code:
//     1. I start my function by initializing the bits integer and the size integer.
//     2. Then I open a while-loop which counts the number of bits and divides 2**size by two, which
//        is why the time complexity is logarithmic. 
//     3. Then I return the number of bits.