package leetcode_grind;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Stack;
import java.util.stream.Collectors;

public class Day318 {
    // https://leetcode.com/problems/remove-duplicate-letters/description
    class Solution1 {
        public String removeDuplicateLettersGreedy(String s) {
            var cnt = new int[26];
            for (int i = 0; i < s.length(); i++) {
                cnt[s.charAt(i) - 'a']++;
            }

            var pos = 0;
            for (int i = 0; i < s.length(); i++) {
                if (s.charAt(i) < s.charAt(pos)) {
                    pos = i;
                }
                if (--cnt[s.charAt(i) - 'a'] == 0) {
                    break;
                }
            }

            if (s.length() == 0) {
                return "";
            }
            return s.charAt(pos)
                    + removeDuplicateLettersGreedy(s.substring(pos + 1).replaceAll("" + s.charAt(pos), ""));
        }

        public String removeDuplicateLettersWithStack(String s) {
            var freq = new char[26];
            for (var ch : s.toCharArray()) {
                freq[ch - 'a']++;
            }

            var onStack = new HashSet<Character>();
            var stack = new Stack<Character>();

            for (var ch : s.toCharArray()) {
                if (!onStack.contains(ch)) {
                    while (!stack.isEmpty() && freq[stack.get(stack.size() - 1) - 'a'] > 0
                            && stack.get(stack.size() - 1) >= ch) {
                        var c = stack.pop();
                        onStack.remove(c);
                    }
                    stack.push(ch);
                    onStack.add(ch);
                }
                freq[ch - 'a']--;
            }

            return stack.stream().map(ch -> Character.toString(ch)).collect(Collectors.joining());
        }
    }

    // https://leetcode.com/problems/design-add-and-search-words-data-structure/description/
    static class WordDictionary {
        static class TrieNode {
            Map<Character, TrieNode> children = new HashMap<>();
            boolean word;
        }

        TrieNode root;

        public WordDictionary() {
            root = new TrieNode();
        }

        public void addWord(String word) {
            var cur = root;
            for (var ch : word.toCharArray()) {
                cur.children.putIfAbsent(ch, new TrieNode());
                cur = cur.children.get(ch);
            }
            cur.word = true;
        }

        public boolean search(String word) {
            var curStack = new LinkedList<TrieNode>();
            curStack.push(root);

            for (var ch : word.toCharArray()) {
                var nextStack = new LinkedList<TrieNode>();
                while (curStack.size() > 0) {
                    var cur = curStack.pop();
                    if (ch == '.') {
                        for (var next : cur.children.values()) {
                            nextStack.push(next);
                        }
                    } else {
                        if (cur.children.containsKey(ch)) {
                            nextStack.push(cur.children.get(ch));
                        }
                    }
                }
                curStack = nextStack;
            }

            while (curStack.size() > 0) {
                if (curStack.pop().word) {
                    return true;
                }
            }

            return false;
        }
    }

    // https://leetcode.com/problems/two-sum/description/
    class Solution2 {
        public int[] twoSum(int[] nums, int target) {
            var seen = new HashMap<Integer, Integer>();
            for (int i = 0; i < nums.length; i++) {
                var complement = target - nums[i];
                if (seen.containsKey(complement)) {
                    return new int[] { seen.get(complement), i };
                }
                seen.put(nums[i], i);
            }
            return new int[] { -1, -1 };
        }
    }

    // https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/description/
    static class Solution3 {
        public int[] twoSum(int[] numbers, int target) {
            var lo = 0;
            var hi = numbers.length - 1;
            while (lo < hi) {
                var sum = numbers[lo] + numbers[hi];
                if (sum < target) {
                    lo++;
                } else if (sum > target) {
                    hi--;
                } else {
                    return new int[] { lo + 1, hi + 1 };
                }
            }
            return new int[] { -1, -1 };
        }
    }

    // https://leetcode.com/problems/3sum/description
    static class Solution4 {
        public List<List<Integer>> threeSumTwoPointers(int[] nums) {
            @FunctionalInterface
            interface TwoSumII {
                void apply(int i, List<List<Integer>> res);
            }

            TwoSumII twoSumII = (i, res) -> {
                var lo = i + 1;
                var hi = nums.length - 1;
                while (lo < hi) {
                    var sum = nums[i] + nums[lo] + nums[hi];
                    if (sum < 0) {
                        lo++;
                    } else if (sum > 0) {
                        hi--;
                    } else {
                        res.add(Arrays.asList(nums[i], nums[lo++], nums[hi--]));
                        while (lo < hi && nums[lo] == nums[lo - 1]) {
                            lo++;
                        }
                    }
                }
            };

            Arrays.sort(nums);
            var res = new ArrayList<List<Integer>>();
            for (int i = 0; i < nums.length && nums[i] <= 0; i++) {
                if (i == 0 || nums[i] != nums[i - 1]) {
                    twoSumII.apply(i, res);
                }
            }
            return res;
        }

        public List<List<Integer>> threeSumHashSet(int[] nums) {
            @FunctionalInterface
            interface TwoSum {
                void apply(int i, List<List<Integer>> res);
            }

            TwoSum twoSum = (i, res) -> {
                var seen = new HashSet<Integer>();
                for (int j = i + 1; j < nums.length; j++) {
                    var complement = -nums[i] - nums[j];
                    if (seen.contains(complement)) {
                        res.add(Arrays.asList(nums[i], nums[j], complement));
                        while (j + 1 < nums.length && nums[j] == nums[j + 1]) {
                            j++;
                        }
                    }
                    seen.add(nums[j]);
                }
            };

            Arrays.sort(nums);
            var res = new ArrayList<List<Integer>>();
            for (int i = 0; i < nums.length && nums[i] != 0; i++) {
                if (i == 0 || nums[i] != nums[i - 1]) {
                    twoSum.apply(i, res);
                }
            }
            return res;
        }

        public List<List<Integer>> threeSumNoSort(int[] nums) {
            var res = new HashSet<List<Integer>>();
            var dups = new HashSet<Integer>();
            var seen = new HashMap<Integer, Integer>();

            for (int i = 0; i < nums.length; i++) {
                if (dups.add(nums[i])) {
                    for (int j = i + 1; j < nums.length; j++) {
                        var complement = -nums[i] - nums[j];
                        if (seen.containsKey(complement) && seen.get(complement) == i) {
                            var triplet = Arrays.asList(nums[i], nums[j], complement);
                            Collections.sort(triplet);
                            res.add(triplet);
                        }
                        seen.put(nums[j], i);
                    }
                }
            }
            return new ArrayList<>(res);
        }
    }
}
