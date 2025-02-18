package leetcode_grind;

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

}
