package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day841 {
    // https://leetcode.com/problems/alternating-groups-i/description/
    static class Solution1 {
        public int numberOfAlternatingGroups(int[] colors) {
            int n = colors.length;
            int ans = 0;
            for (int i = 0; i < n; i++) {
                if (colors[(i - 1 + n) % n] != colors[i] && colors[i] != colors[(i + 1) % n]) {
                    ans++;
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/alternating-groups-ii/description/
    static class Solution2 {
        public int numberOfAlternatingGroups(int[] colors, int k) {
            int[] extendedColors = new int[colors.length + k - 1];
            for (int i = 0; i < colors.length; i++) {
                extendedColors[i] = colors[i];
            }
            for (int i = 0; i < k - 1; i++) {
                extendedColors[colors.length + i] = colors[i];
            }

            int length = extendedColors.length;
            int result = 0;

            int left = 0;
            int right = 1;

            while (right < length) {
                if (extendedColors[right] == extendedColors[right - 1]) {
                    left = right;
                    right++;
                    continue;
                }

                right++;

                if (right - left < k)
                    continue;

                result++;
                left++;
            }

            return result;
        }
    }

    static class Solution3 {
        public int numberOfAlternatingGroups(int[] colors, int k) {
            int length = colors.length;
            int result = 0;
            int alternatingElementsCount = 1;
            int lastColor = colors[0];

            for (int index = 1; index < length; index++) {
                if (colors[index] == lastColor) {
                    alternatingElementsCount = 1;
                    lastColor = colors[index];
                    continue;
                }
                alternatingElementsCount++;
                if (alternatingElementsCount >= k) {
                    result++;
                }
                lastColor = colors[index];
            }

            for (int index = 0; index < k - 1; index++) {
                if (colors[index] == lastColor)
                    break;
                alternatingElementsCount++;
                if (alternatingElementsCount >= k) {
                    result++;
                }
                lastColor = colors[index];
            }
            return result;
        }
    }

    static class Solution4 {
        public int numberOfAlternatingGroups(int[] colors, int k) {
            int length = colors.length;
            int result = 0;
            int alternatingElementsCount = 1;
            int lastColor = colors[0];

            for (int i = 1; i < length + k - 1; i++) {
                int index = i % length;
                if (colors[index] == lastColor) {
                    alternatingElementsCount = 1;
                    lastColor = colors[index];
                    continue;
                }

                alternatingElementsCount += 1;
                if (alternatingElementsCount >= k) {
                    result++;
                }

                lastColor = colors[index];
            }

            return result;
        }
    }

    // https://leetcode.com/problems/fruits-into-baskets-iii/description/
    static class Solution5 {
        int[] segTree;

        void constructTree(int[] basket, int st, int end, int n) {
            if (st == end) {
                segTree[n] = basket[st];
                return;
            }
            int mid = (st + end) / 2;
            constructTree(basket, st, mid, n * 2);
            constructTree(basket, mid + 1, end, n * 2 + 1);
            segTree[n] = Math.max(segTree[n * 2], segTree[n * 2 + 1]);
        }

        int search(int n, int st, int end, int k) {
            if (segTree[n] < k)
                return -1;

            if (st == end) {
                segTree[n] = -1;
                return st;
            }

            int mid = (st + end) / 2;
            int pos = -1;

            if (segTree[2 * n] >= k) {
                pos = search(2 * n, st, mid, k);
            } else {
                pos = search(2 * n + 1, mid + 1, end, k);
            }

            segTree[n] = Math.max(segTree[2 * n], segTree[2 * n + 1]);
            return pos;
        }

        public int numOfUnplacedFruits(int[] fruits, int[] baskets) {
            int n = fruits.length;
            segTree = new int[4 * n];
            constructTree(baskets, 0, n - 1, 1);
            int ans = 0;
            for (int i = 0; i < n; i++) {
                int pos = search(1, 0, n - 1, fruits[i]);
                if (pos == -1) {
                    ans++;
                }
            }
            return ans;
        }
    }

    static class Solution6 {
        public int numOfUnplacedFruits(int[] fruits, int[] baskets) {
            int left = 0;
            int maxValue = 0;
            List<Integer> list = new ArrayList<>();
            for (int j = 0; j < baskets.length; j++) {
                list.add(baskets[j]);
                maxValue = Math.max(maxValue, baskets[j]);
            }

            for (int i = 0; i < fruits.length; i++) {
                if (fruits[i] > maxValue) {
                    left++;
                    continue;
                }

                boolean isMatch = false;
                for (int j = 0; j < list.size(); j++) {
                    if (list.get(j) >= fruits[i]) {
                        list.remove(j);
                        isMatch = true;
                        break;
                    }
                }
                if (!isMatch)
                    left++;
            }

            return left;
        }
    }

    // https://leetcode.com/problems/triples-with-bitwise-and-equal-to-zero/description/
    static class Solution7 {
        public int countTriplets(int[] A) {
            int[] count = new int[1 << 16];
            for (int a : A) {
                for (int b : A) {
                    count[a & b]++;
                }
            }
            int res = 0;
            for (int a : A) {
                for (int i = 0; i < count.length; i++) {
                    if ((a & i) == 0) {
                        res += count[i];
                    }
                }
            }
            return res;
        }
    }

    static class Solution8 {
        static class Trie {
            int count = 0;
            Trie[] child = new Trie[2];

            void add(int num) {
                Trie node = this;
                for (int i = 15; i >= 0; i--) {
                    int b = (num >> i) & 1;
                    if (node.child[b] == null)
                        node.child[b] = new Trie();
                    node = node.child[b];
                }
                node.count++;
            }

            int find(Trie node, int num, int i) {
                return node == null ? 0
                        : (i == -1 ? node.count
                                : (find(node.child[0], num, i - 1) +
                                        (((num >> i) & 1) == 0 ? find(node.child[1], num, i - 1) : 0)));
            }

            int countZeroAND(int num) {
                return find(this, num, 15);
            }
        }

        public int countTriplets(int[] A) {
            Trie trie = new Trie();
            for (int a : A) {
                for (int b : A)
                    trie.add(a & b);
            }
            int res = 0;
            for (int a : A)
                res += trie.countZeroAND(a);
            return res;
        }
    }

    static class Solution9 {
        public int countTriplets(int[] A) {
            int[] dp = new int[1 << 16];
            for (int a : A) {
                dp[a]++;
            }
            for (int i = 0; i < 2; i++) {
                int[] next = new int[dp.length];
                for (int a : A)
                    for (int j = 0; j < dp.length; j++)
                        next[a & j] += dp[j];
                dp = next;
            }
            return dp[0];
        }
    }
}
