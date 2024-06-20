class Solution {
    public boolean isPalindrome(int x) {
        String temp = "";
        for (int i = 0; i < String.valueOf(x).length(); i++) {
            temp += String.valueOf(x).charAt(String.valueOf(x).length() - 1 - i);
        }
        return (String.valueOf(x).equals(temp));
    }
}


// The overall time complexity is O(n), where n is the number of characters in the string representing x. 
// 
// The process I went through here is that I knew I could convert the integer to a string, and compare itself with its
// reversed string, making the overall time complexity O(n). Here is the breakdown of my code:
//     1. I start by initializing the temp string which I will be used at the end of the code.
//     2. Then I open a for loop which will loop through all characters of the string version of x.
//     3. In the for loop, I add to the temp string all characters from the string version of x but in reverse.
//     4. Finally I return the boolean expression comparing the string version of x and its reversed string temp.