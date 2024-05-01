package leetcode_grind;

public class Day535 {
    // https://leetcode.com/problems/reverse-prefix-of-word/description
    static class Solution1 {
        public String reversePrefix(String word, char ch) {
            char[] result = word.toCharArray();
            int left = 0;
            var swap = new Object() {
                void apply(int left, int right) {
                    char t = result[left];
                    result[left] = result[right];
                    result[right] = t;
                }
            };
            for (int right = 0; right < word.length(); right++) {
                if (result[right] == ch) {
                    while (left <= right) {
                        swap.apply(left, right);
                        left++;
                        right--;
                    }
                    return new String(result);
                }
            }
            return word;
        }
    }
}
