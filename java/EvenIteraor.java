import java.util.Iterator;
import java.util.List;

class EvenIterator implements Iterator<Integer> {
    private Iterator<Integer> iter;
    private Integer next;

    public EvenIterator(Iterator<Integer> iter) {
        this.iter = iter;
    }

    @Override
    public boolean hasNext() {
        if (next == null) {
            next = findNext();
        }
        return next != null;
    }

    @Override
    public Integer next() {
        hasNext();
        var t = next;
        next = null;
        return t;
    }

    private Integer findNext() {
        while (iter.hasNext()) {
            var next = iter.next();
            if (next % 2 == 0) {
                return next;
            }
        }
        return null;
    }

    public static void main(String[] args) {
        var list = List.of(1, 1, 2, 3, 3, 4, 5, 5, 6, 7, 7, 8, 9, 9);
        var iter = new EvenIterator(list.iterator());
        while (iter.hasNext()) {
            System.out.println(iter.next());
        }
        System.out.println(iter.hasNext());
        System.out.println(iter.next());
    }
}