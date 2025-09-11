package leetcode_grind;

public class Day1027 {
    // https://leetcode.com/problems/sort-vowels-in-a-string/description/?envType=daily-question&envId=2025-09-11
    static class Solution1 {
        boolean isVowel(Character c) {
            return c == 'a' || c == 'e' || c == 'o' || c == 'u' || c == 'i' ||
                    c == 'A' || c == 'E' || c == 'O' || c == 'U' || c == 'I';
        }

        public String sortVowels(String s) {
            int[] count = new int[1000];

            for (char c : s.toCharArray()) {
                if (isVowel(c)) {
                    count[c - 'A']++;
                }
            }

            String sortedVowel = "AEIOUaeiou";
            StringBuilder ans = new StringBuilder();
            int j = 0;
            for (int i = 0; i < s.length(); i++) {
                if (!isVowel(s.charAt(i))) {
                    ans.append(s.charAt(i));
                } else {
                    while (count[sortedVowel.charAt(j) - 'A'] == 0) {
                        j++;
                    }
                    ans.append(sortedVowel.charAt(j));
                    count[sortedVowel.charAt(j) - 'A']--;
                }
            }
            return ans.toString();
        }
    }
}
