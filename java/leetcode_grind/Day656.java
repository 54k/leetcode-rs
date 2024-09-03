package leetcode_grind;

public class Day656 {
    // https://leetcode.com/problems/sum-of-digits-of-string-after-convert/description/?envType=daily-question&envId=2024-09-03
    static class Solution1 {
        public int getLucky(String s, int k) {
            int currentNumber = 0;
            for (char ch : s.toCharArray()) {
                int position = ch - 'a' + 1;
                while (position > 0) {
                    currentNumber += position % 10;
                    position /= 10;
                }
            }
            for (int i = 1; i < k; i++) {
                int digitSum = 0;
                while (currentNumber > 0) {
                    digitSum += currentNumber % 10;
                    currentNumber /= 10;
                }
                currentNumber = digitSum;
            }
            return currentNumber;
        }
    }

    static class Solution2 {
        public int getLucky(String s, int k) {
            String numericString = "";
            for (char ch : s.toCharArray()) {
                numericString += Integer.toString(ch - 'a' + 1);
            }
            while (k-- > 0) {
                int digitSum = 0;
                for (char digit : numericString.toCharArray()) {
                    digitSum += digit - '0';
                }
                numericString = Integer.toString(digitSum);
            }
            return Integer.parseInt(numericString);
        }
    }
}
