class Solution {
    public int lengthOfLastWord(String s) {
        s = s.trim();
        int num = 0;
        for (int x = 0; x < s.length(); x++) {
            if (s.charAt(s.length() - 1 - x) == ' ') {
                break;
            }
            num += 1;
        }
        return num;
    }
}


// The overall time complexity is O(n), where n is the number of characters in the input string. 
// 
// The process I went through here is knew I could simply strip the string and split it up, and count the number 
// of characters in the last word, which ended up making the overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing my trimed string s and my counter num.
//     2. Then I open a for-loop which adds 1 to my counter num until it reaches the white character 
//        after the last character of the final word in s.
//     3. Then I return num.
