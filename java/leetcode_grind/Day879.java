package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day879 {
    // https://leetcode.com/problems/brace-expansion/description/
    static class Solution1 {
        List<List<Character>> allOptions = new ArrayList<>();

        void storeAllOptions(String s) {
            for (int pos = 0; pos < s.length(); pos++) {
                List<Character> currOptions = new ArrayList<>();
                if (s.charAt(pos) != '{') {
                    currOptions.add(s.charAt(pos));
                } else {
                    while (s.charAt(pos) != '}') {
                        if (s.charAt(pos) >= 'a' && s.charAt(pos) <= 'z') {
                            currOptions.add(s.charAt(pos));
                        }
                        pos++;
                    }
                    Collections.sort(currOptions);
                }

                allOptions.add(currOptions);
            }
        }

        void generateWords(StringBuilder currString, List<String> expandedWords) {
            if (currString.length() == allOptions.size()) {
                expandedWords.add(currString.toString());
                return;
            }

            List<Character> currOptions = allOptions.get(currString.length());

            for (char c : currOptions) {
                currString.append(c);
                generateWords(currString, expandedWords);
                currString.deleteCharAt(currString.length() - 1);
            }
        }

        public String[] expand(String s) {
            storeAllOptions(s);
            List<String> expandedWords = new ArrayList<>();
            generateWords(new StringBuilder(), expandedWords);
            return expandedWords.toArray(new String[0]);
        }
    }

    // https://leetcode.com/problems/count-the-number-of-good-subarrays/description/
    static class Solution2 {
        public long countGood(int[] nums, int k) {
            int n = nums.length;
            int same = 0, right = -1;
            Map<Integer, Integer> cnt = new HashMap<>();
            long ans = 0;

            for (int left = 0; left < n; ++left) {
                while (same < k && right + 1 < n) {
                    ++right;
                    same += cnt.getOrDefault(nums[right], 0);
                    cnt.put(nums[right], cnt.getOrDefault(nums[right], 0) + 1);
                }
                if (same >= k) {
                    ans += n - right;
                }
                cnt.put(nums[left], cnt.get(nums[left]) - 1);
                same -= cnt.get(nums[left]);
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/count-of-smaller-numbers-after-self/description/
    static class Solution3 {
        public List<Integer> countSmaller(int[] nums) {
            int offset = 10000;
            int size = 2 * 10000 + 1;
            int[] tree = new int[size * 2];
            List<Integer> result = new ArrayList<>();

            for (int i = nums.length - 1; i >= 0; i--) {
                int smaller_count = query(0, nums[i] + offset, tree, size);
                result.add(smaller_count);
                update(nums[i] + offset, 1, tree, size);
            }
            Collections.reverse(result);
            return result;
        }

        void update(int index, int value, int[] tree, int size) {
            index += size;
            tree[index] += value;
            while (index > 1) {
                index /= 2;
                tree[index] = tree[index * 2] + tree[index * 2 + 1];
            }
        }

        int query(int left, int right, int[] tree, int size) {
            int result = 0;
            left += size;
            right += size;

            while (left < right) {
                if (left % 2 == 1) {
                    result += tree[left];
                    left++;
                }
                left /= 2;
                if (right % 2 == 1) {
                    right--;
                    result += tree[right];
                }
                right /= 2;
            }
            return result;
        }
    }

    static class Solution4 {
        public List<Integer> countSmaller(int[] nums) {
            int offset = 10000;
            int size = 2 * 10000 + 2;
            int[] tree = new int[size];
            List<Integer> result = new ArrayList<>();
            for (int i = nums.length - 1; i >= 0; i--) {
                int smaller_count = query(nums[i] + offset, tree);
                result.add(smaller_count);
                update(nums[i] + offset, 1, tree, size);
            }
            Collections.reverse(result);
            return result;
        }

        void update(int index, int value, int[] tree, int size) {
            index++;
            while (index < size) {
                tree[index] += value;
                index += index & -index;
            }
        }

        int query(int index, int[] tree) {
            int result = 0;
            while (index >= 1) {
                result += tree[index];
                index -= index & -index;
            }
            return result;
        }
    }

    static class Solution5 {
        public List<Integer> countSmaller(int[] nums) {
            int n = nums.length;
            int[] result = new int[n];
            int[] indices = new int[n];

            for (int i = 0; i < n; i++) {
                indices[i] = i;
            }

            mergeSort(indices, 0, n, result, nums);

            List<Integer> resultToReturn = new ArrayList<>();
            for (int i : result) {
                resultToReturn.add(i);
            }

            return resultToReturn;
        }

        void mergeSort(int[] indices, int left, int right, int[] result, int[] nums) {
            if (right - left <= 1) {
                return;
            }

            int mid = (left + right) / 2;
            mergeSort(indices, left, mid, result, nums);
            mergeSort(indices, mid, right, result, nums);
            merge(indices, left, right, mid, result, nums);
        }

        void merge(int[] indices, int left, int right, int mid, int[] result, int[] nums) {
            int i = left;
            int j = mid;
            List<Integer> temp = new ArrayList<>(right - left);

            while (i < mid && j < right) {
                if (nums[indices[i]] <= nums[indices[j]]) {
                    result[indices[i]] += j - mid;
                    temp.add(indices[i]);
                    i++;
                } else {
                    temp.add(indices[j]);
                    j++;
                }
            }

            while (i < mid) {
                result[indices[i]] += j - mid;
                temp.add(indices[i]);
                i++;
            }

            while (j < right) {
                temp.add(indices[j]);
                j++;
            }

            for (int k = left; k < right; k++) {
                indices[k] = temp.get(k - left);
            }
        }
    }
}
