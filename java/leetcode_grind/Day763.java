package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.Set;
import java.util.Stack;

public class Day763 {
    // https://leetcode.com/problems/maximum-number-of-k-divisible-components/description/?envType=daily-question&envId=2024-12-21
    static class Solution1 {
        public int maxKDivisibleComponents(int n, int[][] edges, int[] values, int k) {
            List<Integer>[] adjList = new ArrayList[n];
            for (int i = 0; i < n; i++) {
                adjList[i] = new ArrayList<>();
            }
            for (int[] edge : edges) {
                int node1 = edge[0];
                int node2 = edge[1];
                adjList[node1].add(node2);
                adjList[node2].add(node1);
            }
            int[] componentCount = new int[1];
            dfs(0, -1, adjList, values, k, componentCount);
            return componentCount[0];
        }

        int dfs(
                int currentNode,
                int parentNode,
                List<Integer>[] adjList,
                int[] nodeValues,
                int k,
                int[] componentCount) {
            int sum = 0;
            for (int neighborNode : adjList[currentNode]) {
                if (neighborNode != parentNode) {
                    sum += dfs(neighborNode, currentNode, adjList, nodeValues, k, componentCount);
                    sum %= k;
                }
            }
            sum += nodeValues[currentNode];
            sum %= k;
            if (sum % k == 0) {
                componentCount[0]++;
            }
            return sum;
        }
    }

    static class Solution2 {
        public int maxKDivisibleComponents(int n, int[][] edges, int[] values, int k) {
            if (n < 2)
                return 1;
            int componentCount = 0;
            Map<Integer, Set<Integer>> graph = new HashMap<>();
            for (int[] edge : edges) {
                int node1 = edge[0], node2 = edge[1];
                graph.computeIfAbsent(node1, key -> new HashSet<>()).add(node2);
                graph.computeIfAbsent(node2, key -> new HashSet<>()).add(node1);
            }

            long[] longValues = new long[values.length];
            for (int i = 0; i < values.length; i++) {
                longValues[i] = values[i];
            }

            Queue<Integer> queue = new LinkedList<>();
            for (Map.Entry<Integer, Set<Integer>> entry : graph.entrySet()) {
                if (entry.getValue().size() == 1) {
                    queue.add(entry.getKey());
                }
            }

            while (!queue.isEmpty()) {
                int currentNode = queue.poll();
                int neighborNode = -1;
                if (graph.get(currentNode) != null && !graph.get(currentNode).isEmpty()) {
                    neighborNode = graph.get(currentNode).iterator().next();
                }

                if (neighborNode >= 0) {
                    graph.get(neighborNode).remove(currentNode);
                    graph.get(currentNode).remove(neighborNode);
                }

                if (longValues[currentNode] % k == 0) {
                    componentCount++;
                } else if (neighborNode >= 0) {
                    longValues[neighborNode] += longValues[currentNode];
                }

                if (neighborNode >= 0 && graph.get(neighborNode) != null && graph.get(neighborNode).size() == 1) {
                    queue.add(neighborNode);
                }
            }

            return componentCount;
        }
    }

    static class Solution3 {
        public int maxKDivisibleComponents(int n, int[][] edges, int[] values, int k) {
            if (n < 2)
                return 1;
            int componentCount = 0;
            List<List<Integer>> graph = new ArrayList<>();
            int[] inDegree = new int[n];

            for (int i = 0; i < n; i++) {
                graph.add(new ArrayList<>());
            }
            for (int[] edge : edges) {
                int node1 = edge[0], node2 = edge[1];
                graph.get(node1).add(node2);
                graph.get(node2).add(node1);
                inDegree[node1]++;
                inDegree[node2]++;
            }

            long[] longValues = new long[n];
            for (int i = 0; i < n; i++) {
                longValues[i] = values[i];
            }

            Queue<Integer> queue = new LinkedList<>();
            for (int node = 0; node < n; node++) {
                if (inDegree[node] == 1) {
                    queue.offer(node);
                }
            }

            while (!queue.isEmpty()) {
                int currentNode = queue.poll();
                inDegree[currentNode]--;

                long addValue = 0;

                if (longValues[currentNode] % k == 0) {
                    componentCount++;
                } else {
                    addValue = longValues[currentNode];
                }

                for (int neighborNode : graph.get(currentNode)) {
                    if (inDegree[neighborNode] == 0) {
                        continue;
                    }
                    inDegree[neighborNode]--;
                    longValues[neighborNode] += addValue;
                    if (inDegree[neighborNode] == 1) {
                        queue.offer(neighborNode);
                    }
                }
            }

            return componentCount;
        }
    }

    // https://leetcode.com/problems/web-crawler/description/
    // /**
    // * // This is the HtmlParser's API interface.
    // * // You should not implement it, or speculate about its implementation
    // * interface HtmlParser {
    // * public List<String> getUrls(String url) {}
    // * }
    // */

    // static class Solution4 {
    // Set<String> vis = new HashSet<>();

    // public List<String> crawl(String startUrl, HtmlParser htmlParser) {
    // var res = new ArrayList<String>();
    // if (!vis.add(startUrl)) {
    // return res;
    // }
    // res.add(startUrl);
    // for (var url : htmlParser.getUrls(startUrl)) {
    // if
    // (url.split("//")[1].split("/")[0].equals(startUrl.split("//")[1].split("/")[0]))
    // {
    // res.addAll(crawl(url, htmlParser));
    // }
    // }
    // return res;
    // }
    // }

    // static class Solution5 {
    // String getHostanme(String url) {
    // return url.split("/")[2];
    // }

    // public List<String> crawl(String startUrl, HtmlParser htmlParser) {
    // String startHostname = getHostanme(startUrl);
    // Queue<String> q = new LinkedList<String>(Arrays.asList(startUrl));
    // HashSet<String> visited = new HashSet<String>(Arrays.asList(startUrl));
    // while (!q.isEmpty()) {
    // String url = q.remove();
    // for (String nextUrl : htmlParser.getUrls(url)) {
    // if (getHostanme(nextUrl).equals(startHostname) && !visited.contains(nextUrl))
    // {
    // q.add(nextUrl);
    // visited.add(nextUrl);
    // }
    // }
    // }
    // return new ArrayList<>(visited);
    // }
    // }

    static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode() {
        }

        TreeNode(int val) {
            this.val = val;
        }

        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }

    static class Pair<F, S> {
        F key;
        S value;

        Pair(F key, S value) {
            this.key = key;
            this.value = value;
        }

        public F getKey() {
            return key;
        }

        public S getValue() {
            return value;
        }
    }

    static class Solution6 {
        public TreeNode str2tree(String s) {
            return str2treeInternal(s, 0).getKey();
        }

        Pair<Integer, Integer> getNumber(String s, int index) {
            boolean isNegative = false;
            if (s.charAt(index) == '-') {
                isNegative = true;
                index++;
            }

            int number = 0;
            while (index < s.length() && Character.isDigit(s.charAt(index))) {
                number = number * 10 + (s.charAt(index) - '0');
                index++;
            }
            return new Pair<Integer, Integer>(isNegative ? -number : number, index);
        }

        Pair<TreeNode, Integer> str2treeInternal(String s, int index) {
            if (index == s.length()) {
                return new Pair<TreeNode, Integer>(null, index);
            }

            Pair<Integer, Integer> numberData = getNumber(s, index);
            int value = numberData.getKey();
            index = numberData.getValue();

            TreeNode node = new TreeNode(value);
            Pair<TreeNode, Integer> data;

            if (index < s.length() && s.charAt(index) == '(') {
                data = this.str2treeInternal(s, index + 1);
                node.left = data.getKey();
                index = data.getValue();
            }

            if (node.left != null && index < s.length() && s.charAt(index) == '(') {
                data = str2treeInternal(s, index + 1);
                node.right = data.getKey();
                index = data.getValue();
            }

            return new Pair<TreeNode, Integer>(node, index < s.length() && s.charAt(index) == ')' ? index + 1 : index);
        }
    }

    static class Solution7 {
        public TreeNode str2tree(String s) {
            if (s.isEmpty()) {
                return null;
            }

            TreeNode root = new TreeNode();
            Stack<TreeNode> stack = new Stack<TreeNode>();
            stack.add(root);

            for (int index = 0; index < s.length();) {
                TreeNode node = stack.pop();

                // NOT STARTED
                if (Character.isDigit(s.charAt(index)) || s.charAt(index) == '-') {
                    Pair<Integer, Integer> numberData = getNumber(s, index);

                    int value = numberData.getKey();
                    index = numberData.getValue();

                    node.val = value;

                    if (index < s.length() && s.charAt(index) == '(') {
                        stack.add(node);
                        node.left = new TreeNode();
                        stack.add(node.left);
                    }
                } else if (s.charAt(index) == '(' && node.left != null) { // LEFT DONE
                    stack.add(node);
                    node.right = new TreeNode();
                    stack.add(node.right);
                }

                ++index;
            }
            return stack.empty() ? root : stack.pop();
        }

        Pair<Integer, Integer> getNumber(String s, int index) {
            boolean isNegative = false;
            if (s.charAt(index) == '-') {
                isNegative = true;
                index++;
            }

            int number = 0;
            while (index < s.length() && Character.isDigit(s.charAt(index))) {
                number = number * 10 + (s.charAt(index) - '0');
                index++;
            }
            return new Pair<Integer, Integer>(isNegative ? -number : number, index);
        }
    }
}
