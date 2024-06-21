impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        return (x.to_string().chars().rev().collect::<String>() == x.to_string());
    }
}


// The overall time complexity is O(n), where n is the number of characters in the string representing x. 
// 
// The process I went through here is that I knew I could convert the integer to a string, and compare itself with its
// reversed string, making the overall time complexity O(n). It's only one line and does the following:
//     a) integer x is converted to a string
//     b) the string is then converted to characters
//     c) the characters are reverse
//     d) the reversed characters are collected into a string
//     e) bool is returned which represents the equality of the reverse string x and original string x