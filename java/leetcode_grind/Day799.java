package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.Set;

public class Day799 {
    // https://leetcode.com/problems/maximum-employees-to-be-invited-to-a-meeting/description/?envType=daily-question&envId=2025-01-26
    static class Solution1 {
        // calc max distance from the given start node
        int bfs(
                int startNode,
                Set<Integer> visitedNodes,
                List<List<Integer>> reversedGraph) {
            Queue<int[]> queue = new LinkedList<>();
            queue.offer(new int[] { startNode, 0 });
            int maxDistance = 0;
            while (!queue.isEmpty()) {
                int[] current = queue.poll();
                int currentNode = current[0];
                int currentDistance = current[1];
                for (int neighbor : reversedGraph.get(currentNode)) {
                    if (visitedNodes.contains(neighbor))
                        continue;

                    visitedNodes.add(neighbor);
                    queue.offer(new int[] { neighbor, currentDistance + 1 });
                    maxDistance = Math.max(maxDistance, currentDistance + 1);
                }
            }
            return maxDistance;
        }

        public int maximumInvitations(int[] favorite) {
            int numPeople = favorite.length;
            List<List<Integer>> reversedGraph = new ArrayList<>(numPeople);
            for (int i = 0; i < numPeople; i++) {
                reversedGraph.add(new ArrayList<>());
            }
            for (int person = 0; person < numPeople; person++) {
                reversedGraph.get(favorite[person]).add(person);
            }

            int longestCycle = 0;
            int twoCycleInvitations = 0;
            boolean[] visited = new boolean[numPeople];

            // Find all cycles in the graph
            for (int person = 0; person < numPeople; person++) {
                if (!visited[person]) {
                    Map<Integer, Integer> visitedPersons = new HashMap<>();
                    int currentPerson = person;
                    int distance = 0;
                    while (true) {
                        if (visited[currentPerson])
                            break;
                        visited[currentPerson] = true;
                        visitedPersons.put(currentPerson, distance++);
                        int nextPerson = favorite[currentPerson];

                        if (visitedPersons.containsKey(nextPerson)) {
                            int cycleLength = distance - visitedPersons.get(nextPerson);
                            longestCycle = Math.max(longestCycle, cycleLength);

                            if (cycleLength == 2) {
                                Set<Integer> visitedNodes = new HashSet<>();
                                visitedNodes.add(currentPerson);
                                visitedNodes.add(nextPerson);
                                twoCycleInvitations += 2 + bfs(nextPerson, visitedNodes, reversedGraph) +
                                        bfs(currentPerson, visitedNodes, reversedGraph);
                            }
                            break;
                        }
                        currentPerson = nextPerson;
                    }
                }
            }

            return Math.max(longestCycle, twoCycleInvitations);
        }
    }
}
