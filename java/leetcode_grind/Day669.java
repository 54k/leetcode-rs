package leetcode_grind;

import java.util.*;

public class Day669 {
    // https://leetcode.com/problems/smallest-common-region/description/?envType=weekly-question&envId=2024-09-15
    static class Solution1 {
        List<String> fetchPathForRegion(
                String currNode,
                Map<String, String> childParentMap) {
            List<String> path = new ArrayList<>();
            path.add(currNode);

            while (childParentMap.containsKey(currNode)) {
                String parentNode = childParentMap.get(currNode);
                path.add(parentNode);
                currNode = parentNode;
            }

            Collections.reverse(path);
            return path;
        }

        public String findSmallestRegion(List<List<String>> regions, String region1, String region2) {
            Map<String, String> childParentMap = new HashMap<>();
            for (List<String> regionArray : regions) {
                String parentNode = regionArray.get(0);
                for (int i = 1; i < regionArray.size(); i++) {
                    childParentMap.put(regionArray.get(i), parentNode);
                }
            }

            List<String> path1 = fetchPathForRegion(region1, childParentMap);
            List<String> path2 = fetchPathForRegion(region2, childParentMap);

            int i = 0, j = 0;
            String lowestCommonAncestor = "";

            while (i < path1.size() && j < path2.size() && path1.get(i).equals(path2.get(j))) {
                lowestCommonAncestor = path1.get(i);
                i++;
                j++;
            }

            return lowestCommonAncestor;
        }
    }

    // https://leetcode.com/problems/minimum-time-difference/description/?envType=daily-question&envId=2024-09-16
    static class Solution2 {
        public int findMinDifference(List<String> timePoints) {
            int[] minutes = new int[timePoints.size()];
            for (int i = 0; i < timePoints.size(); i++) {
                String time = timePoints.get(i);
                int h = Integer.parseInt(time.substring(0, 2));
                int m = Integer.parseInt(time.substring(3));
                minutes[i] = h * 60 + m;
            }
            Arrays.sort(minutes);
            int ans = Integer.MAX_VALUE;
            for (int i = 0; i < minutes.length - 1; i++) {
                ans = Math.min(ans, minutes[i + 1] - minutes[i]);
            }
            return Math.min(ans, 24 * 60 - minutes[minutes.length - 1] + minutes[0]);
        }
    }

    static class Solution3 {
        public int findMinDifference(List<String> timePoints) {
            boolean[] minutes = new boolean[24 * 60];
            for (String time : timePoints) {
                int min = Integer.parseInt(time.substring(0, 2)) * 60 +
                        Integer.parseInt(time.substring(3));
                if (minutes[min])
                    return 0;
                minutes[min] = true;
            }

            int prevIndex = Integer.MAX_VALUE;
            int firstIndex = Integer.MAX_VALUE;
            int lastIndex = Integer.MAX_VALUE;
            int ans = Integer.MAX_VALUE;

            for (int i = 0; i < 24 * 60; i++) {
                if (minutes[i]) {
                    if (prevIndex != Integer.MAX_VALUE) {
                        ans = Math.min(ans, i - prevIndex);
                    }
                    prevIndex = i;
                    if (firstIndex == Integer.MAX_VALUE) {
                        firstIndex = i;
                    }
                    lastIndex = i;
                }
            }

            return Math.min(ans, 24 * 60 - lastIndex + firstIndex);
        }
    }
}
