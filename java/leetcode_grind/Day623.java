package leetcode_grind;

import java.util.Deque;
import java.util.LinkedList;

public class Day623 {
    // https://leetcode.com/problems/number-of-senior-citizens/description/?envType=daily-question&envId=2024-08-01
    static class Solution1 {
        public int countSeniors(String[] details) {
            int seniorCount = 0;

            for (String passengerInfo : details) {
                int ageTens = passengerInfo.charAt(11) - '0';
                int ageOnes = passengerInfo.charAt(12) - '0';

                int age = ageTens * 10 + ageOnes;

                if (age > 60) {
                    seniorCount++;
                }
            }

            return seniorCount;
        }
    }

    // https://leetcode.com/problems/trapping-rain-water/description/
    static class Solution2 {
        public int trap(int[] height) {
            int ans = 0;
            int size = height.length;
            for (int i = 1; i < size - 1; i++) {
                int left_max = 0, right_max = 0;
                for (int j = i; j >= 0; j--) {
                    left_max = Math.max(left_max, height[j]);
                }
                for (int j = i; j < size; j++) {
                    right_max = Math.max(right_max, height[j]);
                }
                ans += Math.min(left_max, right_max) - height[i];
            }
            return ans;
        }
    }

    static class Solution3 {
        public int trap(int[] height) {
            int ans = 0, current = 0;
            Deque<Integer> st = new LinkedList<Integer>();
            while (current < height.length) {
                while (!st.isEmpty() && height[current] > height[st.peek()]) {
                    int top = st.peek();
                    st.pop();
                    if (st.isEmpty())
                        break;
                    int distance = current - st.peek() - 1;
                    int bounded_height = Math.min(height[current], height[st.peek()]) - height[top];
                    ans += distance * bounded_height;
                }
                st.push(current++);
            }
            return ans;
        }
    }
}
