package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;

public class Day331 {

    // https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/description
    static class Solution1 {
        public int[] searchRange(int[] nums, int target) {
            var findBound = new Object() {
                int bound(boolean isFirst) {
                    var lo = 0;
                    var hi = nums.length - 1;
                    while (lo <= hi) {
                        var mid = (lo + hi) / 2;
                        if (nums[mid] == target) {
                            if (isFirst) {
                                if (mid == lo || nums[mid - 1] != target) {
                                    return mid;
                                }

                                hi = mid - 1;
                            } else {
                                if (mid == hi || nums[mid + 1] != target) {
                                    return mid;
                                }

                                lo = mid + 1;
                            }
                        } else if (nums[mid] > target) {
                            hi = mid - 1;
                        } else {
                            lo = mid + 1;
                        }
                    }
                    return -1;
                }
            };

            var firstOccurence = findBound.bound(true);
            if (firstOccurence == -1) {
                return new int[] { -1, -1 };
            }
            var lastOccurence = findBound.bound(false);
            return new int[] { firstOccurence, lastOccurence };
        }
    }

    // https://leetcode.com/problems/serialize-and-deserialize-n-ary-tree/description/
    static class Solution2 {
        class Node {
            public int val;
            public List<Node> children;

            public Node() {
            }

            public Node(int _val) {
                val = _val;
            }

            public Node(int _val, List<Node> _children) {
                val = _val;
                children = _children;
            }
        }

        class Codec {
            static class Pair<F, S> {
                F first;
                S second;

                Pair(F f, S s) {
                    first = f;
                    second = s;
                }
            }

            static class DeserializedObject extends HashMap<Integer, Pair<Integer, Pair<Integer, Node>>> {
            }

            static class WrappableInt {
                int value;

                WrappableInt(int v) {
                    value = v;
                }

                void increment() {
                    value++;
                }
            }

            // Encodes a tree to a single string.
            public String serialize(Node root) {
                var sb = new StringBuilder();
                serializeHelper(root, sb, new WrappableInt(1), null);
                return sb.toString();
            }

            private void serializeHelper(Node root, StringBuilder sb, WrappableInt identity, Integer parentId) {
                if (root == null) {
                    return;
                }

                sb.append((char) (identity.value + '0'));
                sb.append((char) (root.val + '0'));
                sb.append((char) (parentId == null ? 'N' : parentId + '0'));

                parentId = identity.value;
                for (var child : root.children) {
                    identity.increment();
                    serializeHelper(child, sb, identity, parentId);
                }
            }

            // Decodes your encoded data to tree.
            public Node deserialize(String data) {
                if (data.isEmpty()) {
                    return null;
                }
                return deserializeHelper(data);
            }

            private Node deserializeHelper(String data) {
                DeserializedObject nodesAndParents = new DeserializedObject();

                for (int i = 0; i < data.length(); i += 3) {
                    var id = data.charAt(i) - '0';
                    var orgValue = data.charAt(i + 1) - '0';
                    var parentId = data.charAt(i + 2) - '0';

                    var node = new Pair<Integer, Pair<Integer, Node>>(orgValue,
                            new Pair<>(parentId, new Node(orgValue, new ArrayList<>())));
                    nodesAndParents.put(id, node);
                }

                for (int i = 3; i < data.length(); i += 3) {
                    var id = data.charAt(i) - '0';
                    var node = nodesAndParents.get(id).second.second;
                    var parentId = data.charAt(i + 2) - '0';
                    var parentNode = nodesAndParents.get(parentId).second.second;
                    parentNode.children.add(node);
                }

                return nodesAndParents.get(data.charAt(0) - '0').second.second;
            }
        }
    }
}
