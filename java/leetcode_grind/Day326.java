package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Day326 {
    class MyHashMap {
        static class Pair {
            Integer key, val;

            Pair(Integer k, Integer v) {
                key = k;
                val = v;
            }
        }

        static class Bucket {
            private List<Pair> bucket;

            Bucket() {
                this.bucket = new LinkedList<Pair>();
            }

            public Integer get(Integer key) {
                for (var pair : this.bucket) {
                    if (pair.key.equals(key)) {
                        return pair.val;
                    }
                }
                return -1;
            }

            public void update(Integer key, Integer val) {
                var found = false;
                for (var pair : this.bucket) {
                    if (pair.key.equals(key)) {
                        pair.val = val;
                        found = true;
                    }
                }
                if (!found) {
                    this.bucket.add(new Pair(key, val));
                }
            }

            public void remove(Integer key) {
                for (var pair : this.bucket) {
                    if (pair.key.equals(key)) {
                        this.bucket.remove(pair);
                        break;
                    }
                }
            }
        }

        private int keySpace;
        private List<Bucket> hashTable;

        public MyHashMap() {
            keySpace = 2069;
            hashTable = new ArrayList<Bucket>();
            for (var i = 0; i < keySpace; i++) {
                hashTable.add(new Bucket());
            }
        }

        public void put(int key, int value) {
            var hashKey = key % keySpace;
            hashTable.get(hashKey).update(key, value);
        }

        public int get(int key) {
            var hashKey = key % keySpace;
            return hashTable.get(hashKey).get(key);
        }

        public void remove(int key) {
            var hashKey = key % keySpace;
            hashTable.get(hashKey).remove(key);
        }
    }
}
