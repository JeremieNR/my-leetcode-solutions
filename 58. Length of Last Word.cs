public class Solution {
    public int LengthOfLastWord(string s) {
        s = s.Trim();
        String temp = "";
        for (int x = 0; x < s.Length; x++) {
            if (s[s.Length - 1 - x] == ' ') {
                break;
            }
            temp += s[s.Length - 1 - x];
        }
        return temp.Length;
    }
}


// The overall time complexity is O(n), where n is the number of characters in the input string. 
// 
// The process I went through here is knew I could simply strip the string and split it up, and take the length of the last word,
// which ended up making the overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by stripping the ends of the s input string, and initializing the temp string.
//     2. Then I open a for-loop which adds the characters of string s backwards, and stops when it reaches a whitespace.
//     3. Then I return the length of the temp string. 
