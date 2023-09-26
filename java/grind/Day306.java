package grind;
import java.util.*;

class Solution {
    public List<String> findItinerary(List<List<String>> tickets) {
        var adj = new HashMap<String, List<String>>();
        for (var ticket : tickets) {
            adj.putIfAbsent(ticket.get(0), new ArrayList<>(0));
            adj.get(ticket.get(0)).add(ticket.get(1));
        }
        adj.values().forEach(Collections::sort);
        var result = new ArrayList<String>();
        var path = new ArrayList<String>() {
            {
                add("JFK");
            }
        };
        var visited = new HashSet<String>();
        var dfs = new Object() {
            boolean dfs(String from) {
                if (path.size() == tickets.size() + 1) {
                    result.addAll((List<String>) path.clone());
                    return true;
                }
                if (!adj.containsKey(from)) {
                    return false;
                }
                for (var i = 0; i < adj.get(from).size(); i++) {
                    var key = String.format("%s->%s", from, i);
                    if (!visited.contains(key)) {
                        visited.add(key);
                        var to = adj.get(from).get(i);
                        path.add(to);
                        var res = dfs(to);
                        path.remove(path.size() - 1);
                        visited.remove(key);
                        if (res) {
                            return true;
                        }
                    }
                }
                return false;
            }
        };

        dfs.dfs("JFK");
        return result;
    }

    public List<String> findItineraryHirtzholder(List<List<String>> tickets) {
        var adj = new HashMap<String, List<String>>();
        for (var ticket : tickets) {
            adj.putIfAbsent(ticket.get(0), new ArrayList<>());
            adj.get(ticket.get(0)).add(ticket.get(1));
        }

        adj.values().forEach(Collections::sort);

        var path = new ArrayList<String>();
        var visited = new HashSet<String>();

        var dfs = new Object() {
            void dfs(String from) {
                if (adj.containsKey(from)) {
                    for (var i = 0; i < adj.get(from).size(); i++) {
                        var key = String.format("%s->%s", from, i);
                        if (!visited.contains(key)) {
                            visited.add(key);
                            var to = adj.get(from).get(i);
                            dfs(to);
                        }

                    }
                }
                path.add(from);
            }
        };

        dfs.dfs("JFK");
        Collections.reverse(path);
        return path;
    }
}