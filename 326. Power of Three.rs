impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        for exp in 0..20 {
            if ((3 as i32).pow(exp) == n) {
                return true;
            }
        }
        return false;
    }
}


// The overall time complexity is O(1). 
// 
// The process I went through here is that I knew I could use the maximum integer (3^20) as a loop max, making the 
// overall time complexity O(1). I start my function by openning a for-loop, which checks if exp is any power of 3. 
// If it is, it returns true, and if the for-loop ends, it means it isn't and thus returns false.