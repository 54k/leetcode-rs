package leetcode_grind;

import java.util.*;

public class Day706 {
    public static class TreeNode {
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

    // https://leetcode.com/problems/cousins-in-binary-tree-ii/description/?envType=daily-question&envId=2024-10-23
    static class Solution1 {
        public TreeNode replaceValueInTree(TreeNode root) {
            if (root == null) {
                return root;
            }
            Queue<TreeNode> nodeQueue = new LinkedList<>();
            nodeQueue.offer(root);
            int previousLevelSum = root.val;
            while (!nodeQueue.isEmpty()) {
                int levelSize = nodeQueue.size();
                int currentLevelSum = 0;
                for (int i = 0; i < levelSize; i++) {
                    TreeNode currentNode = nodeQueue.poll();
                    currentNode.val = previousLevelSum - currentNode.val;
                    int siblingSum = (currentNode.left != null ? currentNode.left.val : 0) +
                            (currentNode.right != null ? currentNode.right.val : 0);

                    if (currentNode.left != null) {
                        currentLevelSum += currentNode.left.val;
                        currentNode.left.val = siblingSum;
                        nodeQueue.offer(currentNode.left);
                    }
                    if (currentNode.right != null) {
                        currentLevelSum += currentNode.right.val;
                        currentNode.right.val = siblingSum;
                        nodeQueue.offer(currentNode.right);
                    }
                }
                previousLevelSum = currentLevelSum;
            }
            return root;
        }
    }

    static class Solution2 {
        int[] levelSums = new int[100000];

        public TreeNode replaceValueInTree(TreeNode root) {
            calculateLevelSum(root, 0);
            replaceValueInTreeInternal(root, 0, 0);
            return root;
        }

        void calculateLevelSum(TreeNode node, int level) {
            if (node == null) {
                return;
            }
            levelSums[level] += node.val;
            calculateLevelSum(node.left, level + 1);
            calculateLevelSum(node.right, level + 1);
        }

        void replaceValueInTreeInternal(TreeNode node, int siblingSum, int level) {
            if (node == null) {
                return;
            }
            int leftChildVal = (node.left == null) ? 0 : node.left.val;
            int rightChildVal = (node.right == null) ? 0 : node.right.val;
            if (level == 0 || level == 1) {
                node.val = 0;
            } else {
                node.val = levelSums[level] - node.val - siblingSum;
            }
            replaceValueInTreeInternal(node.left, rightChildVal, level + 1);
            replaceValueInTreeInternal(node.right, leftChildVal, level + 1);
        }
    }

    static class Solution3 {
        public TreeNode replaceValueInTree(TreeNode root) {
            if (root == null)
                return root;
            Queue<TreeNode> nodeQueue = new LinkedList<>();
            nodeQueue.offer(root);
            List<Integer> levelSums = new ArrayList<>();

            while (!nodeQueue.isEmpty()) {
                int levelSum = 0;
                int levelSize = nodeQueue.size();
                for (int i = 0; i < levelSize; i++) {
                    TreeNode currentNode = nodeQueue.poll();
                    levelSum += currentNode.val;
                    if (currentNode.left != null)
                        nodeQueue.offer(currentNode.left);
                    if (currentNode.right != null)
                        nodeQueue.offer(currentNode.right);
                }
                levelSums.add(levelSum);
            }

            nodeQueue.offer(root);
            int levelIndex = 1;
            root.val = 0;
            while (!nodeQueue.isEmpty()) {
                int levelSize = nodeQueue.size();
                for (int i = 0; i < levelSize; ++i) {
                    TreeNode currentNode = nodeQueue.poll();
                    int siblingSum = (currentNode.left != null ? currentNode.left.val : 0) +
                            (currentNode.right != null ? currentNode.right.val : 0);

                    if (currentNode.left != null) {
                        currentNode.left.val = levelSums.get(levelIndex) - siblingSum;
                        nodeQueue.offer(currentNode.left);
                    }
                    if (currentNode.right != null) {
                        currentNode.right.val = levelSums.get(levelIndex) - siblingSum;
                        nodeQueue.offer(currentNode.right);
                    }
                }
                ++levelIndex;
            }
            return root;
        }
    }

    // https://leetcode.com/problems/shortest-cycle-in-a-graph/description/
    static class Solution4 {
        public int findShortestCycle(int n, int[][] edges) {
            var adj = new HashMap<Integer, List<Integer>>();
            for (var e : edges) {
                adj.computeIfAbsent(e[0], $ -> new ArrayList<>()).add(e[1]);
                adj.computeIfAbsent(e[1], $ -> new ArrayList<>()).add(e[0]);
            }
            var minCycle = n + 1;

            for (int i = 0; i < n; i++) {
                var dist = new int[n];
                Arrays.fill(dist, -1);
                dist[i] = 0;

                var q = new LinkedList<Integer>();
                q.add(i);
                while (q.size() > 0) {
                    var u = q.remove();
                    for (var v : adj.getOrDefault(u, new ArrayList<>())) {
                        if (dist[v] == -1) {
                            dist[v] = dist[u] + 1;
                            q.add(v);
                        } else if (dist[v] >= dist[u]) {
                            minCycle = Math.min(minCycle, dist[v] + dist[u] + 1);
                        }
                    }
                }
            }

            if (minCycle <= n) {
                return minCycle;
            }
            return -1;
        }
    }

    // https://leetcode.com/problems/word-ladder-ii/description/
    static class Solution5 {
        Map<String, List<String>> adjList = new HashMap<>();
        List<String> currPath = new ArrayList<>();
        List<List<String>> shortestPaths = new ArrayList<>();

        List<String> findNeighbors(String word, Set<String> wordList) {
            List<String> neighbors = new ArrayList<String>();
            char charList[] = word.toCharArray();

            for (int i = 0; i < word.length(); i++) {
                char oldChar = charList[i];
                for (char c = 'a'; c <= 'z'; c++) {
                    charList[i] = c;

                    if (c == oldChar || !wordList.contains(String.valueOf(charList))) {
                        continue;
                    }
                    neighbors.add(String.valueOf(charList));
                }
                charList[i] = oldChar;
            }
            return neighbors;
        }

        void backtrack(String source, String destination) {
            if (source.equals(destination)) {
                List<String> tempPath = new ArrayList<String>(currPath);
                Collections.reverse(tempPath);
                shortestPaths.add(tempPath);
            }

            if (!adjList.containsKey(source)) {
                return;
            }

            for (int i = 0; i < adjList.get(source).size(); i++) {
                currPath.add(adjList.get(source).get(i));
                backtrack(adjList.get(source).get(i), destination);
                currPath.remove(currPath.size() - 1);
            }
        }

        void bfs(String beginWord, String endWord, Set<String> wordList) {
            Queue<String> q = new LinkedList<>();
            q.add(beginWord);

            if (wordList.contains(beginWord)) {
                wordList.remove(beginWord);
            }

            Map<String, Integer> isEnqueued = new HashMap<String, Integer>();
            isEnqueued.put(beginWord, 1);

            while (q.size() > 0) {
                List<String> visited = new ArrayList<String>();
                for (int i = q.size() - 1; i >= 0; i--) {
                    String currWord = q.peek();
                    q.remove();

                    List<String> neighbors = findNeighbors(currWord, wordList);
                    for (String word : neighbors) {
                        visited.add(word);
                        if (!adjList.containsKey(word)) {
                            adjList.put(word, new ArrayList<String>());
                        }
                        adjList.get(word).add(currWord);
                        if (!isEnqueued.containsKey(word)) {
                            q.add(word);
                            isEnqueued.put(word, 1);
                        }
                    }
                }

                for (int i = 0; i < visited.size(); i++) {
                    if (wordList.contains(visited.get(i))) {
                        wordList.remove(visited.get(i));
                    }
                }
            }
        }

        public List<List<String>> findLadders(String beginWord, String endWord, List<String> wordList) {
            Set<String> copiedWordList = new HashSet<>(wordList);
            bfs(beginWord, endWord, copiedWordList);
            currPath.add(endWord);
            backtrack(endWord, beginWord);
            return shortestPaths;
        }
    }

    static class Solution6 {
        Map<String, List<String>> adjList = new HashMap<String, List<String>>();
        List<String> currPath = new ArrayList<String>();
        List<List<String>> shortestPaths = new ArrayList<List<String>>();

        private List<String> findNeighbors(String word, Set<String> wordList) {
            List<String> neighbors = new ArrayList<String>();
            char charList[] = word.toCharArray();

            for (int i = 0; i < word.length(); i++) {
                char oldChar = charList[i];

                // replace the i-th character with all letters from a to z except the original
                // character
                for (char c = 'a'; c <= 'z'; c++) {
                    charList[i] = c;

                    // skip if the character is same as original or if the word is not present in
                    // the wordList
                    if (c == oldChar || !wordList.contains(String.valueOf(charList))) {
                        continue;
                    }
                    neighbors.add(String.valueOf(charList));
                }
                charList[i] = oldChar;
            }
            return neighbors;
        }

        private void backtrack(String source, String destination) {
            // System.out.printlen(source);
            // store the path if we reached the endWord
            if (source.equals(destination)) {
                List<String> tempPath = new ArrayList<String>(currPath);
                shortestPaths.add(tempPath);
            }

            if (!adjList.containsKey(source)) {
                return;
            }

            for (int i = 0; i < adjList.get(source).size(); i++) {
                currPath.add(adjList.get(source).get(i));
                backtrack(adjList.get(source).get(i), destination);
                currPath.remove(currPath.size() - 1);
            }
        }

        private void swap(Set<String> forward, Set<String> backward) {
            Set<String> temp = forward;
            forward = backward;
            backward = temp;
        }

        private void addEdge(String word1, String word2, int direction) {
            if (direction == 1) {
                if (!adjList.containsKey(word1)) {
                    adjList.put(word1, new ArrayList<String>());
                }
                adjList.get(word1).add(word2);
            } else {
                if (!adjList.containsKey(word2)) {
                    adjList.put(word2, new ArrayList<String>());
                }
                adjList.get(word2).add(word1);
            }
        }

        private boolean bfs(
                String beginWord,
                String endWord,
                Set<String> wordList) {
            if (wordList.contains(endWord) == false) {
                return false;
            }

            // remove the root word which is the first layer
            if (wordList.contains(beginWord)) {
                wordList.remove(beginWord);
            }

            Set<String> forwardQueue = new HashSet<String>();
            Set<String> backwardQueue = new HashSet<String>();

            forwardQueue.add(beginWord);
            backwardQueue.add(endWord);

            boolean found = false;
            int direction = 1;

            while (forwardQueue.isEmpty() != true) {
                // visited will store the words of current layer
                Set<String> visited = new HashSet<String>();

                // swap the queues because we are always extending the forwardQueue
                if (forwardQueue.size() > backwardQueue.size()) {
                    Set<String> temp = forwardQueue;
                    forwardQueue = backwardQueue;
                    backwardQueue = temp;
                    direction ^= 1;
                }

                for (String currWord : forwardQueue) {
                    List<String> neighbors = findNeighbors(currWord, wordList);
                    for (String word : neighbors) {
                        // if the backwardQueue already contains it we can stop after completing this
                        // level
                        if (backwardQueue.contains(word)) {
                            found = true;
                            addEdge(currWord, word, direction);
                        } /*
                           * the word shouldn't be presnt in forwardQueue because if it is then the edge
                           * will
                           * be between two words at the same level which we don't want
                           */
                        else if (!found &&
                                wordList.contains(word) == true &&
                                forwardQueue.contains(word) == false) {
                            visited.add(word);
                            addEdge(currWord, word, direction);
                        }
                    }
                }

                // removing the words of the previous layer
                for (String currWord : forwardQueue) {
                    if (wordList.contains(currWord)) {
                        wordList.remove(currWord);
                    }
                }

                if (found) {
                    break;
                }

                forwardQueue = visited;
            }
            return found;
        }

        public List<List<String>> findLadders(
                String beginWord,
                String endWord,
                List<String> wordList) {
            // copying the words into the set for efficient deletion in BFS
            Set<String> copiedWordList = new HashSet<>(wordList);
            // build the DAG using BFS
            boolean sequence_found = bfs(beginWord, endWord, copiedWordList);

            // There is no valid sequence that connects `beginWord` to `endWord`
            if (sequence_found == false) {
                return shortestPaths;
            }
            // every path will start from the beginWord
            currPath.add(beginWord);
            // traverse the DAG to find all the paths between beginWord and endWord
            backtrack(beginWord, endWord);

            return shortestPaths;
        }
    }
}
