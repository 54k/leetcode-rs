package data_structures_examples;

import java.util.ArrayList;
import java.util.List;

public class Heap {
    private final List<Integer> heap;

    public Heap() {
        heap = new ArrayList<Integer>();
        heap.add(0); // 1 index based
    }

    private void swap(int index, int index2) {
        var temp = heap.get(index);
        heap.set(index, heap.get(index2));
        heap.set(index2, temp);
    }

    public void push(Integer val) {
        heap.add(val);
        var index = heap.size() - 1;
        while (index > 1 && heap.get(index) < heap.get(index / 2)) {
            swap(index, index / 2);
            index /= 2;
        }
    }

    public void pop() {
        var index = 1;
        swap(index, heap.size() - 1);
        heap.remove(heap.size() - 1);
        while (index * 2 < heap.size() && heap.get(index) > heap.get(index * 2) ||
                index * 2 + 1 < heap.size() && heap.get(index) > heap.get(index * 2 + 1)) {
            if (index * 2 + 1 < heap.size() && heap.get(index * 2) > heap.get(index * 2 + 1)) {
                swap(index, index * 2 + 1);
                index *= 2;
                index++;
            } else {
                swap(index, index * 2);
                index *= 2;
            }
        }
    }

    public int top() {
        return heap.get(1);
    }

    public static void main(String[] args) {
        var heap = new Heap();

        heap.push(3);
        System.out.println(heap.top());

        heap.push(2);
        System.out.println(heap.top());

        heap.push(1);
        System.out.println(heap.top());

        heap.push(40);
        System.out.println(heap.top());

        heap.pop();
        System.out.println(heap.top());

        heap.pop();
        System.out.println(heap.top());

        heap.pop();
        System.out.println(heap.top());
    }
}
