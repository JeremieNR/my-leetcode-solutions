impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        for x in 0..31 {
            if ((2 as i32).pow(x) == n) {
                return true;
            }
        }
        return false;
    }
}


// The overall time complexity is O(1). 
// 
// The process I went through here is that I knew I could use the maximum integer (2^32) as a loop max, making the 
// overall time complexity O(1). I start my function by openning a for-loop, which checks if n is any power of 2. 
// If it is, it returns true, and if the for-loop ends, it means it isn't and thus returns false.