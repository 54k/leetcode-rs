package leetcode_grind;

import java.util.Iterator;
import java.util.List;
import java.util.Stack;

public class Day342 {
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

    // https://leetcode.com/problems/flatten-nested-list-iterator/
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
