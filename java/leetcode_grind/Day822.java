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
}
