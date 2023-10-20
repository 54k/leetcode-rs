package leetcode_grind;

import java.util.Iterator;
import java.util.List;
import java.util.Stack;

public class Day342 {
    // https://leetcode.com/problems/mini-parser/description/
    static class NestedIteratorProblem {
        public interface NestedInteger {
            // @return true if this NestedInteger holds a single integer, rather than a
            // nested list.
            public boolean isInteger();

            // @return the single integer that this NestedInteger holds, if it holds a
            // single integer
            // Return null if this NestedInteger holds a nested list
            public Integer getInteger();

            // @return the nested list that this NestedInteger holds, if it holds a nested
            // list
            // Return empty list if this NestedInteger holds a single integer
            public List<NestedInteger> getList();
        }

        public class NestedIterator implements Iterator<Integer> {
            Integer ret;
            Stack<Iterator<NestedInteger>> list = new Stack<>();

            public NestedIterator(List<NestedInteger> nestedList) {
                list.push(nestedList.iterator());
            }

            @Override
            public Integer next() {
                hasNext();
                var val = ret;
                ret = null;
                return val;
            }

            @Override
            public boolean hasNext() {
                if (ret == null) {
                    if (list.isEmpty()) {
                        return false;
                    }
                    if (list.peek().hasNext()) {
                        var next = list.peek().next();
                        if (next.isInteger()) {
                            ret = next.getInteger();
                        } else {
                            list.push(next.getList().iterator());
                            hasNext();
                        }
                    } else {
                        list.pop();
                        hasNext();
                    }
                }
                return ret != null;
            }
        }
    }
    
    // https://leetcode.com/problems/mini-parser/description/
    static class NestedListParserProblem {
        public static class NestedInteger {
            // Constructor initializes an empty nested list.
            public NestedInteger() {
            }

            // Constructor initializes a single integer.
            public NestedInteger(int value) {
            }

            // @return true if this NestedInteger holds a single integer, rather than a
            // nested list.
            public boolean isInteger() {
                return false;
            }

            // @return the single integer that this NestedInteger holds, if it holds a
            // single integer
            // Return null if this NestedInteger holds a nested list
            public Integer getInteger() {
                return 0;
            }

            // Set this NestedInteger to hold a single integer.
            public void setInteger(int value) {
            }

            // Set this NestedInteger to hold a nested list and adds a nested integer to it.
            public void add(NestedInteger ni) {
            }

            // @return the nested list that this NestedInteger holds, if it holds a nested
            // list
            // Return empty list if this NestedInteger holds a single integer
            public List<NestedInteger> getList() {
                return List.of();
            }
        }

        static class SolutionRecursive {
            public NestedInteger deserialize(String s) {
                var i = new int[1];
                return solve(s, i).getList().get(0);
            }

            NestedInteger solve(String s, int[] i) {
                NestedInteger result = new NestedInteger();
                Integer currentNum = null;
                Integer sign = 1;

                while (i[0] < s.length()) {
                    var ch = s.charAt(i[0]);
                    if (ch == '-') {
                        sign = -1;
                    } else if (Character.isDigit(ch)) {
                        if (currentNum == null) {
                            currentNum = 0;
                        }
                        currentNum = currentNum * 10 + (ch - '0');
                    } else if (ch == '[') {
                        i[0]++;
                        result.add(solve(s, i));
                    } else if (ch == ',') {
                        if (currentNum != null) {
                            result.add(new NestedInteger(currentNum * sign));
                        }
                        currentNum = null;
                        sign = 1;
                    } else if (ch == ']') {
                        break;
                    }

                    i[0]++;
                }

                if (currentNum != null) {
                    result.add(new NestedInteger(currentNum * sign));
                }

                return result;
            }
        }
    }
}
