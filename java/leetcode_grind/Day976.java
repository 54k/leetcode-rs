package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class Day976 {
    // https://leetcode.com/problems/maximum-erasure-value/description/?envType=daily-question&envId=2025-07-22
    static class Solution1 {
        public int maximumUniqueSubarray(int[] nums) {
            int result = 0;
            int currentSum = 0;
            HashSet<Integer> set = new HashSet<>();
            int start = 0;
            for (int end = 0; end < nums.length; end++) {
                while (set.contains(nums[end])) {
                    set.remove(nums[start]);
                    currentSum -= nums[start];
                    start++;
                }
                currentSum += nums[end];
                set.add(nums[end]);
                result = Math.max(result, currentSum);
            }
            return result;
        }
    }

    static class Solution2 {
        public int maximumUniqueSubarray(int[] nums) {
            int start = 0;
            int result = 0;
            int currentSum = 0;
            int k = 10001;
            int[] countMap = new int[k];
            for (int end = 0; end < nums.length; end++) {
                int currentElement = nums[end];
                countMap[currentElement]++;
                currentSum += currentElement;
                while (start < end && countMap[currentElement] > 1) {
                    countMap[nums[start]]--;
                    currentSum -= nums[start];
                    start++;
                }
                result = Math.max(result, currentSum);
            }
            return result;
        }
    }

    static class Solution3 {
        public int maximumUniqueSubarray(int[] nums) {
            int n = nums.length;
            Map<Integer, Integer> lastIndexMap = new HashMap<>();
            int[] prefixSum = new int[n + 1];

            int result = 0, start = 0;
            for (int end = 0; end < n; end++) {
                int currentElement = nums[end];
                prefixSum[end + 1] = prefixSum[end] + currentElement;
                if (lastIndexMap.containsKey(currentElement)) {
                    start = Math.max(start, lastIndexMap.get(currentElement) + 1);
                }
                result = Math.max(result, prefixSum[end + 1] - prefixSum[start]);
                lastIndexMap.put(currentElement, end);
            }
            return result;
        }
    }

    // https://leetcode.com/problems/word-abbreviation/description/?envType=weekly-question&envId=2025-07-22
    static class Solution4 {
        public List<String> wordsAbbreviation(List<String> words) {
            int N = words.size();
            String[] ans = new String[N];
            int[] prefix = new int[N];

            for (int i = 0; i < N; i++) {
                ans[i] = abbrev(words.get(i), 0);
            }

            for (int i = 0; i < N; i++) {
                while (true) {
                    Set<Integer> dupes = new HashSet<>();
                    for (int j = i + 1; j < N; j++) {
                        if (ans[i].equals(ans[j])) {
                            dupes.add(j);
                        }
                    }

                    if (dupes.isEmpty())
                        break;
                    dupes.add(i);
                    for (int k : dupes) {
                        ans[k] = abbrev(words.get(k), ++prefix[k]);
                    }
                }
            }

            return Arrays.asList(ans);
        }

        String abbrev(String word, int i) {
            int N = word.length();
            if (N - i <= 3)
                return word;
            return word.substring(0, i + 1) + (N - i - 2) + word.charAt(N - 1);
        }
    }

    static class Solution5 {
        static class IndexedWord {
            String word;
            int index;

            IndexedWord(String w, int i) {
                word = w;
                index = i;
            }
        }

        public List<String> wordsAbbreviation(List<String> words) {
            Map<String, List<IndexedWord>> groups = new HashMap<>();
            String[] ans = new String[words.size()];

            int index = 0;
            for (String word : words) {
                String ab = abbrev(word, 0);
                if (!groups.containsKey(ab)) {
                    groups.put(ab, new ArrayList<>());
                }
                groups.get(ab).add(new IndexedWord(word, index));
                index++;
            }

            for (List<IndexedWord> group : groups.values()) {
                Collections.sort(group, (a, b) -> a.word.compareTo(b.word));

                int[] lcp = new int[group.size()];
                for (int i = 1; i < group.size(); ++i) {
                    int p = longestCommonPrefix(group.get(i - 1).word, group.get(i).word);
                    lcp[i] = p;
                    lcp[i - 1] = Math.max(lcp[i - 1], p);
                }

                for (int i = 0; i < group.size(); i++) {
                    ans[group.get(i).index] = abbrev(group.get(i).word, lcp[i]);
                }
            }

            return Arrays.asList(ans);
        }

        String abbrev(String word, int i) {
            int N = word.length();
            if (N - i <= 3)
                return word;
            return word.substring(0, i + 1) + (N - i - 2) + word.charAt(N - 1);
        }

        int longestCommonPrefix(String word1, String word2) {
            int i = 0;
            while (i < word1.length() && i < word2.length() && word1.charAt(i) == word2.charAt(i)) {
                i++;
            }
            return i;
        }
    }

}
