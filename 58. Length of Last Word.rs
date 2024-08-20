impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let trim_s = s.trim();
        let mut num: i32 = 0;
        for x in 0..trim_s.len() {
            if (trim_s.as_bytes()[trim_s.len() - 1 - x] as char == ' ') {
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
//     1. I start my function by initializing my trimed string trim_s and my counter num.
//     2. Then I open a for-loop which adds 1 to my counter num until it reaches the white character 
//        after the last character of the final word in trim_s.
//     3. Then I return num.