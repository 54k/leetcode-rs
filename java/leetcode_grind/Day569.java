package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;

public class Day569 {
    // https://leetcode.com/problems/hand-of-straights/description
    static class Solution1 {
        public boolean isNStraightHand(int[] hand, int groupSize) {
            var cnt = new HashMap<Integer, Integer>();
            for (var v : hand) {
                cnt.put(v, cnt.getOrDefault(v, 0) + 1);
            }
            Arrays.sort(hand);
            for (var ptr = 0; ptr < hand.length; ptr++) {
                if (cnt.getOrDefault(hand[ptr], 0) == 0) {
                    continue;
                }
                for (var j = hand[ptr]; j < hand[ptr] + groupSize; j++) {
                    if (cnt.getOrDefault(j, 0) > 0) {
                        cnt.put(j, cnt.get(j) - 1);
                        // System.out.printf("Take j %s \n", j);
                    } else {
                        // System.out.printf("No candidate j %s found, exit \n", j);
                        return false;
                    }
                }
                // System.out.printf("End forming group \n");
            }
            return true;
        }
    }

    // https://leetcode.com/problems/hand-of-straights/description
    static class Solution2 {
        public boolean isNStraightHand(int[] hand, int groupSize) {
            if (hand.length % groupSize != 0) {
                return false;
            }

            HashMap<Integer, Integer> cardCount = new HashMap<>();
            for (int card : hand) {
                int count = cardCount.getOrDefault(card, 0);
                cardCount.put(card, count + 1);
            }

            for (int card : hand) {
                int startCard = card;
                while (cardCount.getOrDefault(startCard - 1, 0) > 0) {
                    startCard--;
                }
                while (startCard <= card) {
                    while (cardCount.getOrDefault(startCard, 0) > 0) {
                        for (int nextCard = startCard; nextCard < startCard + groupSize; nextCard++) {
                            if (cardCount.getOrDefault(nextCard, 0) == 0) {
                                return false;
                            }
                            cardCount.put(nextCard, cardCount.get(nextCard) - 1);
                        }
                    }
                    startCard++;
                }
            }
            return true;
        }
    }

    // https://leetcode.com/problems/search-in-rotated-sorted-array/description/
    static class Solution3 {
        public int search(int[] nums, int target) {
            int n = nums.length;
            int left = 0, right = n - 1;
            while (left <= right) {
                int mid = (left + right) / 2;
                if (nums[mid] > nums[n - 1]) {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }

            int answer = binarySearch(nums, 0, left - 1, target);
            if (answer != -1) {
                return answer;
            }
            return binarySearch(nums, left, n - 1, target);
        }

        int binarySearch(int[] nums, int leftBoundary, int rightBoundary, int target) {
            int left = leftBoundary, right = rightBoundary;
            while (left <= right) {
                int mid = (left + right) / 2;
                if (nums[mid] == target) {
                    return mid;
                } else if (nums[mid] > target) {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            return -1;
        }
    }
    
    // https://leetcode.com/problems/search-in-rotated-sorted-array/description/
    static class Solution5 {
        public int search(int[] nums, int target) {
            int n = nums.length;
            int left = 0;
            int right = n - 1;
            while (left <= right) {
                int mid = (left + right) / 2;
                if (nums[mid] > nums[n - 1]) {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            return shiftedBinarySearch(nums, left, target);
        }

        int shiftedBinarySearch(int[] nums, int pivot, int target) {
            int n = nums.length;
            int shift = n - pivot;
            int left = (pivot + shift) % n;
            int right = (pivot - 1 + shift) % n;

            while (left <= right) {
                int mid = (left + right) / 2;
                if (nums[(mid - shift + n) % n] == target) {
                    return (mid - shift + n) % n;
                } else if (nums[(mid - shift + n) % n] > target) {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            return -1;
        }
    }
}
