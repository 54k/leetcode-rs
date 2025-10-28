package leetcode_grind;

public class Day1074 {
    // https://leetcode.com/problems/make-array-elements-equal-to-zero/description/?envType=daily-question&envId=2025-10-28
    static class Solution1 {
        public int countValidSelections(int[] nums) {
            int count = 0;
            int nonZeros = 0;
            int n = nums.length;
            for (int x : nums)
                if (x > 0)
                    nonZeros++;
            for (int i = 0; i < n; i++) {
                if (nums[i] == 0) {
                    if (isValid(nums, nonZeros, i, -1))
                        count++;
                    if (isValid(nums, nonZeros, i, 1))
                        count++;
                }
            }
            return count;
        }

        boolean isValid(int[] nums, int nonZeros, int start, int direction) {
            int n = nums.length;
            int curr = start;
            int[] temp = nums.clone();
            while (nonZeros > 0 && curr >= 0 && curr < n) {
                if (temp[curr] > 0) {
                    temp[curr]--;
                    direction *= -1;
                    if (temp[curr] == 0)
                        nonZeros--;
                }
                curr += direction;
            }
            return nonZeros == 0;
        }
    }

    static class Solution2 {
        public int countValidSelections(int[] nums) {
            int n = nums.length;
            int ans = 0;
            int sum = 0;
            for (int x : nums) {
                sum += x;
            }

            int leftSum = 0;
            int rightSum = sum;
            for (int i = 0; i < n; i++) {
                if (nums[i] == 0) {
                    if (leftSum - rightSum >= 0 && leftSum - rightSum <= 1) {
                        ans++;
                    }
                    if (rightSum - leftSum >= 0 && rightSum - leftSum <= 1) {
                        ans++;
                    }
                } else {
                    leftSum += nums[i];
                    rightSum -= nums[i];
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/string-to-integer-atoi/description/
    static class Solution3 {
        public int myAtoi(String s) {
            int sign = 1;
            int result = 0;
            int index = 0;
            int n = s.length();

            while (index < n && s.charAt(index) == ' ') {
                index++;
            }

            if (index < n && s.charAt(index) == '+') {
                sign = 1;
                index++;
            } else if (index < n && s.charAt(index) == '-') {
                sign = -1;
                index++;
            }

            while (index < n && Character.isDigit(s.charAt(index))) {
                int digit = s.charAt(index) - '0';

                if ((result > Integer.MAX_VALUE / 10) ||
                        (result == Integer.MAX_VALUE / 10 && digit > Integer.MAX_VALUE % 10)) {
                    return sign == 1 ? Integer.MAX_VALUE : Integer.MIN_VALUE;
                }

                result = 10 * result + digit;
                index++;
            }

            return sign * result;

        }
    }

    static class Solution4 {
        static enum State {
            q0, q1, q2, qd,
        }

        static class StateMachine {
            State currentState;
            int result, sign;

            StateMachine() {
                currentState = State.q0;
                result = 0;
                sign = 1;
            }

            void toStateQ1(char ch) {
                sign = (ch == '-') ? -1 : 1;
                currentState = State.q1;
            }

            void toStateQ2(int digit) {
                currentState = State.q2;
                appendDigit(digit);
            }

            void toStateQd() {
                currentState = State.qd;
            }

            void appendDigit(int digit) {
                if ((result > Integer.MAX_VALUE / 10) ||
                        (result == Integer.MAX_VALUE / 10 && digit > Integer.MAX_VALUE % 10)) {
                    if (sign == 1) {
                        result = Integer.MAX_VALUE;
                    } else {
                        result = Integer.MIN_VALUE;
                        sign = 1;
                    }

                    toStateQd();
                } else {
                    result = result * 10 + digit;
                }
            }

            void transition(char ch) {
                if (currentState == State.q0) {
                    if (ch == ' ') {
                        return;
                    } else if (ch == '-' || ch == '+') {
                        toStateQ1(ch);
                    } else if (Character.isDigit(ch)) {
                        toStateQ2(ch - '0');
                    } else {
                        toStateQd();
                    }
                } else if (currentState == State.q1 || currentState == State.q2) {
                    if (Character.isDigit(ch)) {
                        toStateQ2(ch - '0');
                    } else {
                        toStateQd();
                    }
                }
            }

            int getInteger() {
                return sign * result;
            }

            State getState() {
                return currentState;
            }
        }

        public int myAtoi(String s) {
            StateMachine Q = new StateMachine();
            for (int i = 0; i < s.length() && Q.getState() != State.qd; ++i) {
                Q.transition(s.charAt(i));
            }
            return Q.getInteger();
        }
    }
}
