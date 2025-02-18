package leetcode_grind;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.Stack;

public class Day822 {
    // https://leetcode.com/problems/construct-smallest-number-from-di-string/description/?envType=daily-question&envId=2025-02-18
    static class Solution1 {
        boolean check(String sequence, String pattern) {
            for (int index = 0; index < pattern.length(); index++) {
                if (pattern.charAt(index) == 'I') {
                    if (sequence.charAt(index) > sequence.charAt(index + 1)) {
                        return false;
                    }
                } else {
                    if (sequence.charAt(index) < sequence.charAt(index + 1)) {
                        return false;
                    }
                }
            }
            return true;
        }

        public String smallestNumber(String pattern) {
            int patternLength = pattern.length();
            char[] numberSequence = new char[patternLength + 1];

            for (int position = 0; position <= patternLength; position++) {
                numberSequence[position] = (char) ('1' + position);
            }

            while (!check(new String(numberSequence), pattern)) {
                if (!findNextPermutation(numberSequence)) {
                    break;
                }
            }
            return new String(numberSequence);
        }

        boolean findNextPermutation(char[] numberSequence) {
            int lastIncreasingIndex = numberSequence.length - 2;
            while (lastIncreasingIndex >= 0
                    && numberSequence[lastIncreasingIndex] >= numberSequence[lastIncreasingIndex + 1]) {
                lastIncreasingIndex--;
            }
            if (lastIncreasingIndex == -1)
                return false;

            int swapIndex = numberSequence.length - 1;
            while (numberSequence[swapIndex] <= numberSequence[lastIncreasingIndex]) {
                swapIndex--;
            }
            swapCharacters(numberSequence, lastIncreasingIndex, swapIndex);
            reverseSuffix(numberSequence, lastIncreasingIndex + 1, numberSequence.length - 1);
            return true;
        }

        void swapCharacters(char[] array, int firstIdx, int secondIdx) {
            char temp = array[firstIdx];
            array[firstIdx] = array[secondIdx];
            array[secondIdx] = temp;
        }

        void reverseSuffix(char[] array, int startIdx, int endIdx) {
            while (startIdx < endIdx) {
                swapCharacters(array, startIdx, endIdx);
                startIdx++;
                endIdx--;
            }
        }
    }

    static class Solution2 {
        public String smallestNumber(String pattern) {
            return String.valueOf(findSmallestNumber(pattern, 0, 0, 0));
        }

        int findSmallestNumber(String pattern, int currentPosition, int usedDigitsMask, int currentNum) {
            if (currentPosition > pattern.length())
                return currentNum;
            int result = Integer.MAX_VALUE;
            int lastDigit = currentNum % 10;
            boolean shouldIncrement = currentPosition == 0 || pattern.charAt(currentPosition - 1) == 'I';
            for (int currentDigit = 1; currentDigit <= 9; currentDigit++) {
                if ((usedDigitsMask & (1 << currentDigit)) == 0 && currentDigit > lastDigit == shouldIncrement) {
                    result = Math.min(result, findSmallestNumber(pattern, currentPosition + 1,
                            usedDigitsMask | (1 << currentDigit), currentNum * 10 + currentDigit));
                }
            }
            return result;
        }
    }

    static class Solution3 {
        public String smallestNumber(String pattern) {
            StringBuilder result = new StringBuilder();
            buildSequence(0, 0, pattern.toCharArray(), result);
            return result.reverse().toString();
        }

        int buildSequence(
                int currentIndex,
                int currentCount,
                char[] patternArray,
                StringBuilder result) {
            if (currentIndex != patternArray.length) {
                if (patternArray[currentIndex] == 'I') {
                    buildSequence(currentIndex + 1, currentIndex + 1, patternArray, result);
                } else {
                    currentCount = buildSequence(currentIndex + 1, currentCount, patternArray, result);
                }
            }
            result.append(currentCount + 1);
            return currentCount + 1;
        }
    }

    static class Solution4 {
        public String smallestNumber(String pattern) {
            StringBuilder result = new StringBuilder();
            Stack<Integer> numStack = new Stack<>();
            for (int index = 0; index <= pattern.length(); index++) {
                numStack.push(index + 1);
                if (index == pattern.length() || pattern.charAt(index) == 'I') {
                    while (!numStack.isEmpty()) {
                        result.append(numStack.pop());
                    }
                }
            }
            return result.toString();
        }
    }

    // https://leetcode.com/problems/palindrome-permutation-ii/description/
    static class Solution5 {
        Set<String> set = new HashSet<>();

        public List<String> generatePalindromes(String s) {
            permute(s.toCharArray(), 0);
            return new ArrayList<>(set);
        }

        boolean isPalindrome(char[] s) {
            for (int i = 0; i < s.length; i++) {
                if (s[i] != s[s.length - 1 - i])
                    return false;
            }
            return true;
        }

        void swap(char[] s, int i, int j) {
            char temp = s[i];
            s[i] = s[j];
            s[j] = temp;
        }

        void permute(char[] s, int l) {
            if (l == s.length) {
                if (isPalindrome(s))
                    set.add(new String(s));
            } else {
                for (int i = l; i < s.length; i++) {
                    swap(s, l, i);
                    permute(s, l + 1);
                    swap(s, l, i);
                }
            }
        }
    }

    static class Solution6 {
        Set<String> set = new HashSet<>();

        public List<String> generatePalindromes(String s) {
            int[] map = new int[128];
            char[] st = new char[s.length() / 2];
            if (!canPermutePalindrome(s, map))
                return new ArrayList<>();
            char ch = 0;
            int k = 0;
            for (int i = 0; i < map.length; i++) {
                if (map[i] % 2 == 1)
                    ch = (char) i;

                for (int j = 0; j < map[i] / 2; j++) {
                    st[k++] = (char) i;
                }
            }

            permute(st, 0, ch);
            return new ArrayList<String>(set);
        }

        boolean canPermutePalindrome(String s, int[] map) {
            int count = 0;
            for (int i = 0; i < s.length(); i++) {
                map[s.charAt(i)]++;
                if (map[s.charAt(i)] % 2 == 0)
                    count--;
                else
                    count++;
            }
            return count <= 1;
        }

        void swap(char[] s, int i, int j) {
            char temp = s[i];
            s[i] = s[j];
            s[j] = temp;
        }

        void permute(char[] s, int l, char ch) {
            if (l == s.length) {
                set.add(new String(s) + (ch == 0 ? "" : ch) + new StringBuffer(new String(s)).reverse());
            } else {
                for (int i = l; i < s.length; i++) {
                    if (s[l] != s[i] || l == i) {
                        swap(s, l, i);
                        permute(s, l + 1, ch);
                        swap(s, l, i);
                    }
                }
            }
        }
    }
}
