package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day645 {
    // https://leetcode.com/problems/fraction-addition-and-subtraction/description/?envType=daily-question&envId=2024-08-23
    static class Solution1 {
        public String fractionAddition(String expression) {
            int num = 0;
            int denom = 1;
            int i = 0;

            while (i < expression.length()) {
                int currNum = 0;
                int currDenom = 0;
                boolean isNegative = false;

                if (expression.charAt(i) == '-' || expression.charAt(i) == '+') {
                    if (expression.charAt(i) == '-') {
                        isNegative = true;
                    }
                    i++;
                }

                while (Character.isDigit(expression.charAt(i))) {
                    int val = expression.charAt(i) - '0';
                    currNum = currNum * 10 + val;
                    i++;
                }

                if (isNegative)
                    currNum *= -1;

                i++;

                while (i < expression.length() && Character.isDigit(expression.charAt(i))) {
                    int val = expression.charAt(i) - '0';
                    currDenom = currDenom * 10 + val;
                    i++;
                }

                num = num * currDenom + currNum * denom;
                denom = denom * currDenom;
            }

            int gcd = Math.abs(findGCD(num, denom));
            num /= gcd;
            denom /= gcd;
            return num + "/" + denom;
        }

        int findGCD(int a, int b) {
            if (a == 0)
                return b;
            return findGCD(b % a, a);
        }
    }

    static class Solution2 {
        public String fractionAddition(String expression) {
            int num = 0;
            int denom = 1;
            String[] nums = expression.split("/|(?=[-+])");
            for (int i = 0; i < nums.length; i += 2) {
                int currNum = Integer.valueOf(nums[i]);
                int currDenom = Integer.valueOf(nums[i + 1]);
                num = num * currDenom + currNum * denom;
                denom = denom * currDenom;
            }
            int gcd = Math.abs(findGCD(num, denom));

            num /= gcd;
            denom /= gcd;
            return num + "/" + denom;
        }

        int findGCD(int a, int b) {
            if (a == 0)
                return b;
            return findGCD(b % a, a);
        }
    }

    // https://leetcode.com/problems/generalized-abbreviation/description/?envType=weekly-question&envId=2024-08-22
    static class Solution3 {
        void storeAbbreviations(List<String> abbreviations, String word, StringBuilder currWord, int index,
                int abbreviatedCount) {
            if (index == word.length()) {
                if (abbreviatedCount > 0) {
                    currWord.append(abbreviatedCount);
                }
                abbreviations.add(currWord.toString());
                return;
            }

            int currWordLength = currWord.length();
            if (abbreviatedCount > 0) {
                currWord.append(abbreviatedCount);
            }

            currWord.append(word.charAt(index));
            storeAbbreviations(abbreviations, word, currWord, index + 1, 0);
            currWord.setLength(currWordLength);
            storeAbbreviations(abbreviations, word, currWord, index + 1, abbreviatedCount + 1);
        }

        public List<String> generateAbbreviations(String word) {
            List<String> abbreviations = new ArrayList<>();
            storeAbbreviations(abbreviations, word, new StringBuilder(), 0, 0);
            return abbreviations;
        }
    }
}
