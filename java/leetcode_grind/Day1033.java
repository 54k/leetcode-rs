package leetcode_grind;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

public class Day1033 {
    // https://leetcode.com/problems/design-task-manager/description/?envType=daily-question&envId=2025-09-18
    static class TaskManager {
        static class Task implements Comparable<Task> {
            int priority;
            int userId;
            int taskId;

            Task(List<Integer> lst) {
                this(lst.get(0), lst.get(1), lst.get(2));
            }

            Task(int userId, int taskId, int priority) {
                this.userId = userId;
                this.taskId = taskId;
                this.priority = priority;
            }

            @Override
            public int compareTo(Task other) {
                if (other.priority == priority) {
                    return other.taskId - taskId;
                }
                return other.priority - this.priority;
            }
        }

        Map<Integer, Task> idToTask = new HashMap<>();

        TreeMap<Integer, Task> tmap = new TreeMap<>((id1, id2) -> {
            return idToTask.get(id1).compareTo(idToTask.get(id2));
        });

        public TaskManager(List<List<Integer>> tasks) {
            for (var lst : tasks) {
                Task task = new Task(lst);
                idToTask.put(task.taskId, task);
                tmap.put(task.taskId, task);
            }
        }

        public void add(int userId, int taskId, int priority) {
            Task task = new Task(userId, taskId, priority);
            idToTask.put(task.taskId, task);
            tmap.put(task.taskId, task);
        }

        public void edit(int taskId, int newPriority) {
            Task t = tmap.remove(taskId);
            t.priority = newPriority;
            tmap.put(t.taskId, t);
        }

        public void rmv(int taskId) {
            tmap.remove(taskId);
            idToTask.remove(taskId);
        }

        public int execTop() {
            if (tmap.isEmpty()) {
                return -1;
            }
            Task t = tmap.firstEntry().getValue();
            tmap.remove(t.taskId);
            idToTask.remove(t.taskId);
            return t.userId;
        }
    }

    /**
     * Your TaskManager object will be instantiated and called as such:
     * TaskManager obj = new TaskManager(tasks);
     * obj.add(userId,taskId,priority);
     * obj.edit(taskId,newPriority);
     * obj.rmv(taskId);
     * int param_4 = obj.execTop();
     */

}
