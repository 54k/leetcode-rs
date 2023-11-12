package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day365 {
    // https://leetcode.com/problems/longest-common-subsequence/description/
    static class Solution1 {
        public int longestCommonSubsequence(String text1, String text2) {
            var dp = new int[text1.length() + 1][text2.length() + 1];
            for (int i = 1; i <= text1.length(); i++) {
                for (int j = 1; j <= text2.length(); j++) {
                    if (text1.charAt(i - 1) == text2.charAt(j - 1)) {
                        dp[i][j] = dp[i - 1][j - 1] + 1;
                    } else {
                        dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1]);
                    }
                }
            }
            return dp[text1.length()][text2.length()];
        }
    }

    // https://leetcode.com/problems/add-strings/description/
    static class Solution2 {
        public String addStrings(String num1, String num2) {
            var res = "";
            var sum = 0;

            var i = num1.length() - 1;
            var j = num2.length() - 1;

            while (true) {
                if (i >= 0) {
                    sum += num1.charAt(i) - '0';
                    i--;
                }
                if (j >= 0) {
                    sum += num2.charAt(j) - '0';
                    j--;
                }

                res += sum % 10;
                sum /= 10;

                if (i < 0 && j < 0) {
                    break;
                }
            }

            if (sum > 0) {
                res += sum;
            }

            var arr = res.toCharArray();
            for (i = 0; i < arr.length / 2; i++) {
                var t = arr[i];
                arr[i] = arr[arr.length - 1 - i];
                arr[arr.length - 1 - i] = t;
            }

            return new String(arr);
        }
    }

    // https://leetcode.com/problems/multiply-strings/description/
    static class Solution3 {
        public StringBuilder sumResults(List<List<Integer>> results) {
            var answer = new ArrayList<Integer>(results.get(results.size() - 1));
            var newAnswer = new ArrayList<Integer>();

            for (int j = 0; j < results.size() - 1; j++) {
                var result = new ArrayList<Integer>(results.get(j));
                newAnswer = new ArrayList<Integer>();

                int carry = 0;
                for (int i = 0; i < answer.size() || i < result.size(); i++) {
                    var digit1 = i < result.size() ? result.get(i) : 0;
                    var digit2 = i < answer.size() ? answer.get(i) : 0;

                    int sum = digit1 + digit2 + carry;
                    carry = sum / 10;

                    newAnswer.add(sum % 10);
                }

                if (carry != 0) {
                    newAnswer.add(carry);
                }
                answer = newAnswer;
            }

            var finalAnswer = new StringBuilder();
            for (var digit : answer) {
                finalAnswer.append(digit);
            }
            return finalAnswer;
        }

        public List<Integer> multiplyOneDigit(StringBuilder firstNumber, char secondNumberDigit, int numZeros) {
            var result = new ArrayList<Integer>();
            for (int i = 0; i < numZeros; i++) {
                result.add(0);
            }

            var curr = 0;
            for (int i = 0; i < firstNumber.length(); i++) {
                curr += (firstNumber.charAt(i) - '0') * (secondNumberDigit - '0');
                result.add(curr % 10);
                curr /= 10;
            }

            if (curr != 0) {
                result.add(curr);
            }

            return result;
        }

        public String multiply(String num1, String num2) {
            if (num1.equals("0") || num2.equals("0")) {
                return "0";
            }

            var firstNumber = new StringBuilder(num1);
            var secondNumber = new StringBuilder(num2);

            firstNumber.reverse();
            secondNumber.reverse();

            var results = new ArrayList<List<Integer>>();
            for (int i = 0; i < secondNumber.length(); i++) {
                results.add(multiplyOneDigit(firstNumber, secondNumber.charAt(i), i));
            }

            var answer = sumResults(results);
            answer.reverse();
            return answer.toString();
        }
    }
}
