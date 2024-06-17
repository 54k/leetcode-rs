package data_structures_examples;


// https://neerc.ifmo.ru/wiki/index.php?title=%D0%90%D0%BB%D0%B3%D0%BE%D1%80%D0%B8%D1%82%D0%BC_%D0%A4%D0%BE%D1%80%D0%B4%D0%B0-%D0%A4%D0%B0%D0%BB%D0%BA%D0%B5%D1%80%D1%81%D0%BE%D0%BD%D0%B0,_%D1%80%D0%B5%D0%B0%D0%BB%D0%B8%D0%B7%D0%B0%D1%86%D0%B8%D1%8F_%D1%81_%D0%BF%D0%BE%D0%BC%D0%BE%D1%89%D1%8C%D1%8E_%D0%BF%D0%BE%D0%B8%D1%81%D0%BA%D0%B0_%D0%B2_%D0%B3%D0%BB%D1%83%D0%B1%D0%B8%D0%BD%D1%83

// Идея алгоритма заключается в следующем. Изначально величине потока присваивается значение 0
// : f(u,v)=0
//  для всех u,v
//  из V
// . Затем величина потока итеративно увеличивается посредством поиска увеличивающего пути (путь от источника s
//  к стоку t
// , вдоль которого можно послать ненулевой поток). В данной статье рассматривается алгоритм, осуществляющий этот поиск с помощью обхода в глубину (dfs). Процесс повторяется, пока можно найти увеличивающий путь.

// int dfs(int u, int Cmin):         // Cmin — пропускная способность в текущем подпотоке
//    if u = t
//        return Cmin
//    visited[u] = true                  
//    for v in u.children
//        auto uv = edge(u, v)
//        if not visited[v] and uv.f < uv.c
//            int delta = dfs(v, min(Cmin, uv.c - uv.f))
//            if delta > 0
//                uv.f += delta
//                uv.backEdge.f -= delta
//                return delta
//    return 0

// граф
class FlowNetwork {
}

class FlowEdge {
    int v; // начальная вершина ребра
    int w; // конечная вершина ребра
    double capacity; // пропускная способность
    double flow; // поток

    FlowEdge(int v, int w, double capacity) {
        this.v = v;
        this.w = w;
        this.capacity = capacity;
        this.flow = 0.0;
    }

    int other(int vertex) {
        if (vertex == v)
            return w;
        return v;
    }

    double residualCapacity(int vertex) {
        if (vertex == v) {
            return flow;
        } else {
            return capacity - flow;
        }
    }

    void addResidualFlowTo(int vertex, double delta) {
        if (vertex == v) {
            flow -= delta;
        } else {
            flow += delta;
        }
    }
}

public class FordFulkerson {
    boolean[] marked; // принадлежит ли путь s->v остаточному графу
    FlowEdge[] edgeTo; // последнее ребро в кратчайшем пути s->v
    double value; // текущее значение максимального потока

    FordFulkerson(FlowNetwork G, int s, int t) {
        while (hasAugmentingPath(G, s, t)) {
            double bottle = Double.POSITIVE_INFINITY;
            // вычисление манимальной пропускной способности
            for (int v = t; v != s; v = edgeTo[v].other(v)) {
                bottle = Math.min(bottle, edgeTo[v].residualCapacity(v));
            }

            // расширение пути
            for (int v = t; v != s; v = edgeTo[v].other(v)) {
                edgeTo[v].addResidualFlowTo(v, bottle);
            }
            value += bottle;
        }
    }

    boolean inCut(int v) {
        return marked[v];
    }

    boolean hasAugmentingPath(FlowNetwork G, int s, int t) {
        marked = new boolean[G.V()]; // известен ли путь к данной вершине
        edgeTo = new FlowEdge[G.v()]; // последнее ребро в пути
        Queue<Integer> q = new Queue<Integer>();
        marked[s] = true;
        q.enqueue(s);

        while (!q.isEmpty()) {
            int v = q.dequeue();

            for (FlowEdge e : G.adj(v)) {
                int w = e.other(v);
                if (e.residualCapacity(w) > 0 && !marked[w]) {
                    edgeTo[w] = e;
                    marked[w] = true;
                    q.enqueue(w);
                }
            }
        }

        return marked[t];
    }

    public static void main() {
        FlowNetwork G = new FlowNetwork(new In(args[0]));
        int s = 0, t = G.V() - 1; // исток = 0, сток = n-1

        FordFulkerson maxflow = new FordFulkerson(G, s, t);
        StdOut.println("Максимальный опток из " + s + " в " + t);

        for (int v = 0; v < G.V(); v++) {
            for (FlowEdge e : G.adj(v)) {
                if (v == e.from() && e.flow() > 0) {
                    StdOut.println(" " + e);
                }
            }
        }
        StdOut.println("Величина максимального потока = " + maxflow.value());
    }
}