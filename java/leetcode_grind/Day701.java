package leetcode_grind;

import java.util.*;

public class Day701 {
    // https://leetcode.com/problems/maximum-swap/description/?envType=daily-question&envId=2024-10-17
    static class Solution1 {
        public int maximumSwap(int num) {
            var ans = num;
            var arr = Integer.toString(num).toCharArray();
            for (int i = 0; i < arr.length - 1; i++) {
                for (int j = i + 1; j < arr.length; j++) {
                    if (arr[i] < arr[j]) {
                        swap(arr, i, j);
                        var next = Integer.valueOf(new String(arr));
                        if (next > ans) {
                            ans = next;
                        }
                        swap(arr, i, j);
                    }
                }
            }
            return ans;
        }

        void swap(char[] arr, int i, int j) {
            var t = arr[i];
            arr[i] = arr[j];
            arr[j] = t;
        }
    }

    static class Solution2 {
        public int maximumSwap(int num) {
            char[] numArr = Integer.toString(num).toCharArray();
            int n = numArr.length;
            int[] maxRightIndex = new int[n];

            maxRightIndex[n - 1] = n - 1;
            for (int i = n - 2; i >= 0; i--) {
                maxRightIndex[i] = (numArr[i] > numArr[maxRightIndex[i + 1]]) ? i : maxRightIndex[i + 1];
            }

            for (int i = 0; i < n; ++i) {
                if (numArr[i] < numArr[maxRightIndex[i]]) {
                    char temp = numArr[i];
                    numArr[i] = numArr[maxRightIndex[i]];
                    numArr[maxRightIndex[i]] = temp;
                    return Integer.parseInt(new String(numArr));
                }
            }

            return num;
        }
    }

    static class Solution3 {
        public int maximumSwap(int num) {
            String numStr = Integer.toString(num);
            int n = numStr.length();
            int[] lastSeen = new int[10];

            for (int i = 0; i < n; i++) {
                lastSeen[numStr.charAt(i) - '0'] = i;
            }

            for (int i = 0; i < n; i++) {
                for (int d = 9; d > numStr.charAt(i) - '0'; --d) {
                    if (lastSeen[d] > i) {
                        char[] arr = numStr.toCharArray();
                        char temp = arr[i];
                        arr[i] = arr[lastSeen[d]];
                        arr[lastSeen[d]] = temp;
                        numStr = new String(arr);
                        return Integer.parseInt(numStr);
                    }
                }
            }
            return num;
        }
    }

    static class Solution4 {
        public int maximumSwap(int num) {
            char[] numStr = Integer.toString(num).toCharArray();
            int n = numStr.length;
            int maxDigitIndex = -1, swapIdx1 = -1, swapIdx2 = -1;

            for (int i = n - 1; i >= 0; i--) {
                if (maxDigitIndex == -1 || numStr[i] > numStr[maxDigitIndex]) {
                    maxDigitIndex = i;
                } else if (numStr[i] < numStr[maxDigitIndex]) {
                    swapIdx1 = i;
                    swapIdx2 = maxDigitIndex;
                }
            }

            if (swapIdx1 != -1 && swapIdx2 != -1) {
                char temp = numStr[swapIdx1];
                numStr[swapIdx1] = numStr[swapIdx2];
                numStr[swapIdx2] = temp;
            }

            return Integer.parseInt(new String(numStr));
        }
    }

    // https://leetcode.com/problems/largest-component-size-by-common-factor/description/
    static class Solution5 {
        static class DistjoinSetUnion {
            int[] parent;
            int[] size;

            DistjoinSetUnion(int size) {
                parent = new int[size + 1];
                this.size = new int[size + 1];
                for (int i = 0; i < size + 1; i++) {
                    parent[i] = i;
                    this.size[i] = 1;
                }
            }

            int find(int x) {
                if (parent[x] != x) {
                    parent[x] = find(parent[x]);
                }
                return parent[x];
            }

            int union(int x, int y) {
                int px = find(x);
                int py = find(y);
                if (px == py) {
                    return px;
                }

                if (size[px] > size[py]) {
                    int temp = px;
                    px = py;
                    py = temp;
                }

                parent[px] = py;
                size[py] += size[px];
                return py;
            }
        }

        public int largestComponentSize(int[] A) {
            int maxValue = Arrays.stream(A).reduce(A[0], (x, y) -> x > y ? x : y);
            DistjoinSetUnion dsu = new DistjoinSetUnion(maxValue);
            for (int num : A) {
                for (int factor = 2; factor < (int) (Math.sqrt(num)) + 1; ++factor) {
                    if (num % factor == 0) {
                        dsu.union(num, factor);
                        dsu.union(num, num / factor);
                    }
                }
            }

            int maxGroupSize = 0;
            Map<Integer, Integer> groupCount = new HashMap<>();
            for (int num : A) {
                Integer groupId = dsu.find(num);
                Integer count = groupCount.getOrDefault(groupId, 0);
                groupCount.put(groupId, count + 1);
                maxGroupSize = Math.max(maxGroupSize, count + 1);
            }
            return maxGroupSize;
        }
    }

    // https://leetcode.com/problems/longest-cycle-in-a-graph/description/
    static class Solution6 {
        int answer = -1;

        void dfs(int node, int[] edges, Map<Integer, Integer> dist, boolean[] visit) {
            visit[node] = true;
            int neighbor = edges[node];
            if (neighbor != -1 && !visit[neighbor]) {
                dist.put(neighbor, dist.get(node) + 1);
                dfs(neighbor, edges, dist, visit);
            } else if (neighbor != -1 && dist.containsKey(neighbor)) {
                answer = Math.max(answer, dist.get(node) - dist.get(neighbor) + 1);
            }
        }

        public int longestCycle(int[] edges) {
            int n = edges.length;
            boolean[] visit = new boolean[n];

            for (int i = 0; i < n; i++) {
                if (!visit[i]) {
                    Map<Integer, Integer> dist = new HashMap<>();
                    dist.put(i, 1);
                    dfs(i, edges, dist, visit);
                }
            }

            return answer;
        }
    }

    // https://leetcode.com/problems/reorganize-string/description/
    static class Solution7 {
        public String reorganizeString(String s) {
            var pq = new PriorityQueue<int[]>((a, b) -> b[0] - a[0]);
            var cnt = new int[26];
            for (char c : s.toCharArray()) {
                cnt[c - 'a']++;
            }
            for (var i = 0; i < 26; i++) {
                if (cnt[i] > 0) {
                    pq.add(new int[] { cnt[i], i });
                }
            }

            var sb = new StringBuilder();
            while (!pq.isEmpty()) {
                var ok = false;
                var lst = new ArrayList<int[]>();
                do {
                    if (pq.isEmpty()) {
                        return "";
                    }
                    var top = pq.remove();
                    if (sb.length() > 0 && (top[1] + 'a') == sb.charAt(sb.length() - 1)) {
                        lst.add(top);
                    } else {
                        ok = true;
                        sb.append((char) (top[1] + 'a'));
                        top[0]--;
                        if (top[0] > 0) {
                            pq.add(top);
                        }
                        for (var c : lst) {
                            pq.add(c);
                        }
                    }
                } while (!ok);
            }
            return sb.toString();
        }
    }

    static class Solution8 {
        public String reorganizeString(String s) {
            var charCounts = new int[26];
            for (char c : s.toCharArray()) {
                charCounts[c - 'a']++;
            }

            var pq = new PriorityQueue<int[]>((a, b) -> Integer.compare(b[1], a[1]));
            for (int i = 0; i < 26; i++) {
                if (charCounts[i] > 0) {
                    pq.offer(new int[] { i + 'a', charCounts[i] });
                }
            }

            var sb = new StringBuilder();
            while (!pq.isEmpty()) {
                var first = pq.poll();
                if (sb.length() == 0 || first[0] != sb.charAt(sb.length() - 1)) {
                    sb.append((char) first[0]);
                    if (--first[1] > 0) {
                        pq.offer(first);
                    }
                } else {
                    if (pq.isEmpty()) {
                        return "";
                    }
                    var second = pq.poll();
                    sb.append((char) second[0]);
                    if (--second[1] > 0) {
                        pq.offer(second);
                    }
                    pq.offer(first);
                }
            }
            return sb.toString();
        }
    }

    static class Solution9 {
        public String reorganizeString(String s) {
            var charCounts = new int[26];
            for (char c : s.toCharArray()) {
                charCounts[c - 'a']++;
            }

            int maxCount = 0, letter = 0;
            for (int i = 0; i < charCounts.length; i++) {
                if (charCounts[i] > maxCount) {
                    maxCount = charCounts[i];
                    letter = i;
                }
            }

            if (maxCount > (s.length() + 1) / 2) {
                return "";
            }
            var ans = new char[s.length()];
            int index = 0;

            while (charCounts[letter] != 0) {
                ans[index] = (char) (letter + 'a');
                index += 2;
                charCounts[letter]--;
            }

            for (int i = 0; i < charCounts.length; i++) {
                while (charCounts[i] > 0) {
                    if (index >= s.length()) {
                        index = 1;
                    }
                    ans[index] = (char) (i + 'a');
                    index += 2;
                    charCounts[i]--;
                }
            }
            return String.valueOf(ans);
        }
    }

    // https://leetcode.com/problems/rearrange-string-k-distance-apart/description/
    static class Solution10 {
        public String rearrangeString(String s, int k) {
            Map<Character, Integer> freq = new HashMap<>();
            for (char c : s.toCharArray()) {
                freq.put(c, freq.getOrDefault(c, 0) + 1);
            }

            var free = new PriorityQueue<int[]>((a, b) -> Integer.compare(b[0], a[0]));
            for (char c : freq.keySet()) {
                free.offer(new int[] { freq.get(c), c });
            }

            var ans = new StringBuilder();
            var busy = new LinkedList<int[]>();

            while (ans.length() != s.length()) {
                int index = ans.length();
                if (!busy.isEmpty() && (index - busy.peek()[0]) >= k) {
                    var q = busy.remove();
                    free.offer(new int[] { freq.get((char) q[1]), q[1] });
                }

                if (free.isEmpty()) {
                    return "";
                }

                var currChar = (char) free.peek()[1];
                free.remove();
                ans.append(currChar);

                freq.put(currChar, freq.get(currChar) - 1);
                if (freq.get(currChar) > 0) {
                    busy.add(new int[] { index, currChar });
                }
            }
            return ans.toString();
        }
    }

    static class Solution11 {
        public String rearrangeString(String s, int k) {
            Map<Character, Integer> freqs = new HashMap<>();
            int maxFreq = 0;
            for (char c : s.toCharArray()) {
                freqs.put(c, freqs.getOrDefault(c, 0) + 1);
                maxFreq = Math.max(maxFreq, freqs.get(c));
            }

            Set<Character> mostChars = new HashSet<>();
            Set<Character> secondChars = new HashSet<>();

            for (char c : freqs.keySet()) {
                if (freqs.get(c) == maxFreq) {
                    mostChars.add(c);
                } else if (freqs.get(c) == maxFreq - 1) {
                    secondChars.add(c);
                }
            }

            StringBuilder[] segments = new StringBuilder[maxFreq];
            for (int i = 0; i < maxFreq; i++) {
                segments[i] = new StringBuilder();
                for (char c : mostChars) {
                    segments[i].append(c);
                }
                if (i < maxFreq - 1) {
                    for (char c : secondChars) {
                        segments[i].append(c);
                    }
                }
            }

            int segmentId = 0;
            for (char c : freqs.keySet()) {
                if (mostChars.contains(c) || secondChars.contains(c)) {
                    continue;
                }

                for (int freq = freqs.get(c); freq > 0; freq--) {
                    segments[segmentId].append(c);
                    segmentId = (segmentId + 1) % (maxFreq - 1);
                }
            }

            for (int i = 0; i < maxFreq - 1; i++) {
                if (segments[i].length() < k) {
                    return "";
                }
            }

            return String.join("", segments);
        }
    }
}
