package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.Set;

public class Day387 {
    // https://leetcode.com/problems/word-ladder-ii/description/
    // TLE
    static class Solution1 {

        Map<String, List<String>> adjList = new HashMap<String, List<String>>();
        List<String> currPath = new ArrayList<String>();
        List<List<String>> shortestPaths = new ArrayList<List<String>>();

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

        void swap(Set<String> forward, Set<String> backward) {
            Set<String> temp = forward;
            forward = backward;
            backward = temp;
        }

        void backtrack(String source, String destination) {
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

        void addEdge(String word1, String word2, int direction) {
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

        boolean bfs(String beginWord, String endWord, Set<String> wordList) {
            if (!wordList.contains(endWord)) {
                return false;
            }

            if (wordList.contains(beginWord)) {
                wordList.remove(beginWord);
            }

            Set<String> forwardQueue = new HashSet<>();
            Set<String> backwardQueue = new HashSet<>();

            forwardQueue.add(beginWord);
            backwardQueue.add(endWord);

            boolean found = false;
            int direction = 1;

            while (!forwardQueue.isEmpty()) {
                Set<String> visited = new HashSet<String>();

                if (forwardQueue.size() > backwardQueue.size()) {
                    Set<String> temp = forwardQueue;
                    forwardQueue = backwardQueue;
                    backwardQueue = temp;
                    direction ^= 1;
                }

                for (String currWord : forwardQueue) {
                    List<String> neighbors = findNeighbors(currWord, wordList);
                    for (String word : neighbors) {

                        if (backwardQueue.contains(word)) {
                            found = true;
                            addEdge(currWord, word, direction);
                        } else if (!found && wordList.contains(word) && !forwardQueue.contains(word)) {
                            visited.add(word);
                            addEdge(currWord, word, direction);
                        }
                    }
                }

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

        public List<List<String>> findLadders(String beginWord, String endWord, List<String> wordList) {
            Set<String> copiedWordList = new HashSet<>(wordList);
            boolean sequenceFound = bfs(beginWord, endWord, copiedWordList);

            if (sequenceFound == false) {
                return shortestPaths;
            }
            currPath.add(beginWord);
            backtrack(beginWord, endWord);
            return shortestPaths;
        }
    }

    // https://leetcode.com/problems/word-ladder/description/
    static class Solution2 {
        private static class Pair<F, S> {
            F key;
            S value;

            Pair(F k, S v) {
                key = k;
                value = v;
            }

            F getKey() {
                return key;
            }

            S getValue() {
                return value;
            }
        }

        public int ladderLength(String beginWord, String endWord, List<String> wordList) {
            int L = beginWord.length();
            Map<String, List<String>> allComboDict = new HashMap<>();

            wordList.forEach(word -> {
                for (int i = 0; i < L; i++) {
                    String newWord = word.substring(0, i) + '*' + word.substring(i + 1, L);
                    List<String> transformations = allComboDict.getOrDefault(newWord, new ArrayList<>());
                    transformations.add(word);
                    allComboDict.put(newWord, transformations);
                }
            });

            Queue<Pair<String, Integer>> Q = new LinkedList<>();
            Q.add(new Pair(beginWord, 1));

            Set<String> visited = new HashSet<>();
            visited.add(beginWord);

            while (!Q.isEmpty()) {
                Pair<String, Integer> node = Q.remove();
                String word = node.getKey();
                int level = node.getValue();
                for (int i = 0; i < L; i++) {
                    String newWord = word.substring(0, i) + '*' + word.substring(i + 1, L);

                    for (String adjacentWord : allComboDict.getOrDefault(newWord, new ArrayList<>())) {
                        if (adjacentWord.equals(endWord)) {
                            return level + 1;
                        }
                        if (!visited.contains(adjacentWord)) {
                            visited.add(adjacentWord);
                            Q.add(new Pair(adjacentWord, level + 1));
                        }
                    }
                }
            }

            return 0;
        }
    }

    // https://leetcode.com/problems/buddy-strings/description/
    static class Solution {
        public boolean buddyStrings(String s, String goal) {
            if (s.length() != goal.length()) {
                return false;
            }

            if (s.equals(goal)) {
                int[] freq = new int[26];
                for (int i = 0; i < s.length(); i++) {
                    char cur = s.charAt(i);
                    freq[cur - 'a']++;
                    if (freq[cur - 'a'] == 2) {
                        return true;
                    }
                }
                return false;
            }

            int firstIndex = -1;
            int secondIndex = -1;
            for (int i = 0; i < s.length(); i++) {
                if (s.charAt(i) != goal.charAt(i)) {
                    if (firstIndex == -1) {
                        firstIndex = i;
                    } else if (secondIndex == -1) {
                        secondIndex = i;
                    } else {
                        return false;
                    }
                }
            }
            if (secondIndex == -1) {
                return false;
            }
            return s.charAt(firstIndex) == goal.charAt(secondIndex) && s.charAt(secondIndex) == goal.charAt(firstIndex);
        }
    }
}
