package leetcode_grind;

public class Day435 {
    // https://leetcode.com/problems/compare-version-numbers/description/
    static class Solution1 {
        public int compareVersion1(String version1, String version2) {
            String[] nums1 = version1.split("\\.");
            String[] nums2 = version2.split("\\.");
            int n1 = nums1.length, n2 = nums2.length;

            int i1, i2;
            for (int i = 0; i < Math.max(n1, n2); ++i) {
                i1 = i < n1 ? Integer.parseInt(nums1[i]) : 0;
                i2 = i < n2 ? Integer.parseInt(nums2[i]) : 0;

                if (i1 != i2) {
                    return i1 > i2 ? 1 : -1;
                }
            }

            return 0;
        }

        static class Pair<F, S> {
            F first;
            S second;

            Pair(F f, S s) {
                first = f;
                second = s;
            }
        }

        Pair<Integer, Integer> getNextChunk(String version, int n, int p) {
            if (p > n - 1) {
                return new Pair<>(0, p);
            }

            int i, pEnd = p;

            while (pEnd < n && version.charAt(pEnd) != '.') {
                ++pEnd;
            }

            if (pEnd != n - 1) {
                i = Integer.parseInt(version.substring(p, pEnd));
            } else {
                i = Integer.parseInt(version.substring(p, n));
            }

            p = pEnd + 1;
            return new Pair<>(i, p);
        }

        public int compareVersion(String version1, String version2) {
            int p1 = 0, p2 = 0;
            int n1 = version1.length(), n2 = version2.length();

            int i1, i2;
            Pair<Integer, Integer> pair;

            while (p1 < n1 || p2 < n2) {
                pair = getNextChunk(version1, n1, p1);
                i1 = pair.first;
                p1 = pair.second;

                pair = getNextChunk(version2, n2, p2);
                i2 = pair.first;
                p2 = pair.second;

                if (i1 != i2) {
                    return i1 > i2 ? 1 : -1;
                }
            }

            return 0;
        }
    }
}
