import java.util.PriorityQueue;

class MaxAvgPassRatio1792 {
    public double ratio(int[] clazz) {
        var ratioBefore = (double) clazz[0] / (double) clazz[1];
        var ratioAfter = ((double) clazz[0] + 1.0) / ((double) clazz[1] + 1.0);
        return ratioAfter - ratioBefore;
    }
    
    public double maxAverageRatio(int[][] classes, int extraStudents) {
        var pq = new PriorityQueue<int[]>((a, b) -> {
            var cmp = ratio(a) - ratio(b);
            if (cmp < 0.0) {
                return 1;
            } else if (cmp > 0.0) {
                return -1;
            }
            return 0;
        });
        for (var e: classes) {
            pq.add(e);
        }
        while (extraStudents > 0) {
            extraStudents--;
            var clazz = pq.remove();
            clazz[0]++;
            clazz[1]++;
            pq.add(clazz);
        }
        var ar = pq.stream().map((x) -> (double) x[0] / (double) x[1]).collect(Collectors.toList());
        var result = 0.0;
        for (var p : ar) {
            result += p;
        } 
        return result / classes.length;
    }
}