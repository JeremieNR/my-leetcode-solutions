class Solution {
    public boolean isAnagram(String s, String t) {
        Map<Character, Integer> hm1 = new HashMap<Character, Integer>();
        Map<Character, Integer> hm2 = new HashMap<Character, Integer>();

        if (s.length() != t.length()) {
            return false;
        }

        for (int x = 0; x < s.length(); x++) {
            if (hm1.containsKey(s.charAt(x))) {
                hm1.put(s.charAt(x), hm1.get(s.charAt(x)) + 1);
            } else {
                hm1.put(s.charAt(x), 0);
            } if (hm2.containsKey(t.charAt(x))) {
                hm2.put(t.charAt(x), hm2.get(t.charAt(x)) + 1);
            } else {
                hm2.put(t.charAt(x), 0);
            }
        }
        return (hm1.equals(hm2));
    }
}


// The overall time complexity is O(n), where n is the number of characters in the input string s/t. 
// 
// The process I went through here is that I knew I could use two hashmaps so I can traverse on of the strings once, 
// making the overall time complexity O(n). Here is the breakdown of my code:
//     1. I start my function by initializing both hashmaps as hm1 and hm2.
//     2. Then I return false if the length of the strings aren't equal.
//     3. Then I open a for-loop which indexes all number of characters there are in string s and t.
//     4. Then I return the equality of both hashmaps to check if the strings are valid anagrams.
