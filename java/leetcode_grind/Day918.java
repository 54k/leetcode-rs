package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day918 {
    // https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/description/?envType=daily-question&envId=2025-05-25
    static class Solution1 {
        public int longestPalindrome(String[] words) {
            Map<String, Integer> count = new HashMap<>();
            for (String word : words) {
                Integer countOfTheWord = count.get(word);
                if (countOfTheWord == null) {
                    count.put(word, 1);
                } else {
                    count.put(word, countOfTheWord + 1);
                }
            }

            int answer = 0;
            boolean central = false;
            for (Map.Entry<String, Integer> entry : count.entrySet()) {
                String word = entry.getKey();
                int countOfTheWord = entry.getValue();
                if (word.charAt(0) == word.charAt(1)) {
                    if (countOfTheWord % 2 == 0) {
                        answer += countOfTheWord;
                    } else {
                        answer += countOfTheWord - 1;
                        central = true;
                    }
                } else if (word.charAt(0) < word.charAt(1)) {
                    String reversedWord = "" + word.charAt(1) + word.charAt(0);
                    if (count.containsKey(reversedWord)) {
                        answer += 2 * Math.min(countOfTheWord, count.get(reversedWord));
                    }
                }
            }
            if (central) {
                answer++;
            }
            return answer * 2;
        }
    }

    static class Solution2 {
        public int longestPalindrome(String[] words) {
            int alphabetSize = 26;
            int[][] count = new int[alphabetSize][alphabetSize];
            for (String word : words) {
                count[word.charAt(0) - 'a'][word.charAt(1) - 'a']++;
            }

            int answer = 0;
            boolean central = false;
            for (int i = 0; i < alphabetSize; i++) {
                if (count[i][i] % 2 == 0) {
                    answer += count[i][i];
                } else {
                    answer += count[i][i] - 1;
                    central = true;
                }
                for (int j = i + 1; j < alphabetSize; j++) {
                    answer += 2 * Math.min(count[i][j], count[j][i]);
                }
            }
            if (central) {
                answer++;
            }
            return answer * 2;
        }
    }
}
