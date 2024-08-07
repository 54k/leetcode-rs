package leetcode_grind;

import java.util.Arrays;
import java.util.List;

public class Day629 {
    // https://leetcode.com/problems/integer-to-english-words/description/?envType=daily-question&envId=2024-08-07
    static class Solution1 {
        // Arrays to store words for numbers less than 10, 20, and 100
        private static final String[] belowTen = { "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
                "Nine" };
        private static final String[] belowTwenty = { "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen",
                "Sixteen", "Seventeen", "Eighteen", "Nineteen" };
        private static final String[] belowHundred = { "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty",
                "Seventy", "Eighty", "Ninety" };

        public String numberToWords(int num) {
            if (num == 0)
                return "Zero";
            return convertToWords(num);
        }

        String convertToWords(int num) {
            if (num < 10)
                return belowTen[num];
            if (num < 20)
                return belowTwenty[num - 10];
            if (num < 100) {
                return belowHundred[num / 10] + (num % 10 != 0 ? " " + convertToWords(num % 10) : "");
            }
            if (num < 1000) {
                return convertToWords(num / 100) + " Hundred" + (num % 100 != 0 ? " " + convertToWords(num % 100) : "");
            }
            if (num < 1_000_000) {
                return convertToWords(num / 1000) + " Thousand"
                        + (num % 1000 != 0 ? " " + convertToWords(num % 1000) : "");
            }
            if (num < 1_000_000_000) {
                return convertToWords(num / 1000_000) + " Million"
                        + (num % 1_000_000 != 0 ? " " + convertToWords(num % 1_000_000) : "");
            }
            return convertToWords(num / 1_000_000_000) + " Billion"
                    + (num % 1_000_000_000 != 0 ? " " + convertToWords(num % 1_000_000_000) : "");
        }
    }

    static class Solution2 {
        public String numberToWords(int num) {
            // Handle the special case where the number is zero
            if (num == 0)
                return "Zero";

            // Arrays to store words for single digits, tens, and thousands
            String[] ones = { "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
                    "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen",
                    "Nineteen" };
            String[] tens = { "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety" };
            String[] thousands = { "", "Thousand", "Million", "Billion" };

            StringBuilder result = new StringBuilder();
            int groupIndex = 0;

            while (num > 0) {
                if (num % 1000 != 0) {
                    StringBuilder groupResult = new StringBuilder();
                    int part = num % 1000;
                    if (part >= 100) {
                        groupResult.append(ones[part / 100]).append(" Hundred ");
                        part %= 100;
                    }
                    if (part >= 20) {
                        groupResult.append(tens[part / 10]).append(" ");
                        part %= 10;
                    }
                    if (part > 0) {
                        groupResult.append(ones[part]).append(" ");
                    }

                    groupResult.append(thousands[groupIndex]).append(" ");
                    result.insert(0, groupResult);
                }
                num /= 1000;
                groupIndex++;
            }
            return result.toString().trim();
        }
    }

    static class Solution3 {

        static class NumberWord {
            int value;
            String word;

            NumberWord(int value, String word) {
                this.value = value;
                this.word = word;
            }
        }

        private static final List<NumberWord> numberToWordsList = Arrays.asList(
                new NumberWord(1000000000, "Billion"), new NumberWord(1000000, "Million"),
                new NumberWord(1000, "Thousand"), new NumberWord(100, "Hundred"),
                new NumberWord(90, "Ninety"), new NumberWord(80, "Eighty"),
                new NumberWord(70, "Seventy"), new NumberWord(60, "Sixty"),
                new NumberWord(50, "Fifty"), new NumberWord(40, "Forty"),
                new NumberWord(30, "Thirty"), new NumberWord(20, "Twenty"),
                new NumberWord(19, "Nineteen"), new NumberWord(18, "Eighteen"),
                new NumberWord(17, "Seventeen"), new NumberWord(16, "Sixteen"),
                new NumberWord(15, "Fifteen"), new NumberWord(14, "Fourteen"),
                new NumberWord(13, "Thirteen"), new NumberWord(12, "Twelve"),
                new NumberWord(11, "Eleven"), new NumberWord(10, "Ten"),
                new NumberWord(9, "Nine"), new NumberWord(8, "Eight"),
                new NumberWord(7, "Seven"), new NumberWord(6, "Six"),
                new NumberWord(5, "Five"), new NumberWord(4, "Four"),
                new NumberWord(3, "Three"), new NumberWord(2, "Two"),
                new NumberWord(1, "One"));

        public String numberToWords(int num) {
            if (num == 0) {
                return "Zero";
            }

            for (NumberWord nw : numberToWordsList) {
                if (num >= nw.value) {
                    String prefix = (num >= 100) ? numberToWords(num / nw.value) + " " : "";
                    String unit = nw.word;

                    String suffix = (num % nw.value == 0) ? "" : " " + numberToWords(num % nw.value);
                    return prefix + unit + suffix;
                }
            }

            return "";
        }
    }
}
