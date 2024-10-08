package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Stack;

public class Day692 {
    // https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/description/?envType=daily-question&envId=2024-10-08
    static class Solution1 {
        public int minSwaps(String s) {
            Stack<Character> stack = new Stack<>();
            int unbalanced = 0;
            for (int i = 0; i < s.length(); i++) {
                char ch = s.charAt(i);
                if (ch == '[')
                    stack.push(ch);
                else {
                    if (!stack.isEmpty())
                        stack.pop();
                    else
                        unbalanced++;
                }
            }
            return (unbalanced + 1) / 2;
        }
    }

    // https://leetcode.com/problems/maximum-coins-heroes-can-collect/description/?envType=weekly-question&envId=2024-10-08
    static class Solution2 {
        public long[] maximumCoins(int[] heroes, int[] monsters, int[] coins) {
            long[] ans = new long[heroes.length];
            int[][] monsterAndCoin = new int[monsters.length][2];
            for (int i = 0; i < monsters.length; i++) {
                monsterAndCoin[i][0] = monsters[i];
                monsterAndCoin[i][1] = coins[i];
            }
            Arrays.sort(monsterAndCoin, (a, b) -> a[0] - b[0]);

            long[] coinSum = new long[coins.length];
            long prefixSum = 0;
            for (int i = 0; i < monsterAndCoin.length; i++) {
                prefixSum += monsterAndCoin[i][1];
                coinSum[i] = prefixSum;
            }

            for (int i = 0; i < heroes.length; i++) {
                ans[i] = findTotalCoins(monsterAndCoin, heroes[i], coinSum);
            }
            return ans;
        }

        long findTotalCoins(int[][] monsterAndCoin, int heroPower, long[] coinSum) {
            int l = 0;
            int r = monsterAndCoin.length - 1;
            while (l <= r) {
                int mid = (l + r) / 2;
                if (monsterAndCoin[mid][0] > heroPower) {
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            }
            if (l == 0 && monsterAndCoin[l][0] > heroPower) {
                return 0;
            }
            return coinSum[r];
        }
    }

    // https://leetcode.com/problems/delete-duplicate-folders-in-system/description/
    static class Solution3 {

        static class Folder {
            String name;
            Map<String, Folder> map;
            List<Folder> list;
            String key;
            boolean del;

            Folder(String name) {
                this.name = name;
                map = new HashMap<>();
                list = new ArrayList<>();
                key = "";
                del = false;
            }
        }

        Folder root = new Folder("");
        Map<String, Integer> keys = new HashMap<>();

        public List<List<String>> deleteDuplicateFolder(List<List<String>> paths) {
            for (List<String> path : paths) {
                addPath(path);
            }
            for (Folder f : root.list) {
                generateKey(f);
            }
            for (Folder f : root.list) {
                updateDeleteStatus(f);
            }
            List<List<String>> results = new ArrayList<>();
            for (List<String> path : paths) {
                if (isValid(path)) {
                    results.add(path);
                }
            }
            return results;
        }

        boolean isValid(List<String> path) {
            Folder current = root;
            for (String f : path) {
                current = current.map.get(f);
                if (current.del) {
                    return false;
                }
            }
            return true;
        }

        void updateDeleteStatus(Folder f) {
            if (f.list.size() > 0 && keys.get(f.key) > 1) {
                f.del = true;
                return;
            }
            for (Folder fold : f.list) {
                updateDeleteStatus(fold);
            }
        }

        String generateKey(Folder fold) {
            StringBuilder sb = new StringBuilder();
            if (fold.list.size() == 0) {
                return sb.toString();
            }
            Collections.sort(fold.list, (a, b) -> a.name.compareTo(b.name));

            for (Folder f : fold.list) {
                sb.append('(');
                sb.append(f.name + generateKey(f));
                sb.append(')');
            }

            String key = sb.toString();
            fold.key = key;
            keys.put(key, keys.getOrDefault(key, 0) + 1);
            return key;
        }

        void addPath(List<String> path) {
            Folder current = root;
            for (String f : path) {
                if (!current.map.containsKey(f)) {
                    Folder fold = new Folder(f);
                    current.map.put(f, fold);
                    current.list.add(fold);
                }
                current = current.map.get(f);
            }
        }
    }
}
