from typing import List

class Solution:
    def findMedian(self, n: int, edges: List[List[int]], queries: List[List[int]]) -> List[int]:
        adj = [[] for _ in range(n)]
        for u, v, w in edges:
            adj[u].append((v, w))
            adj[v].append((u, w))

        LOG = 20
        up = [[-1] * LOG for _ in range(n)]
        dist = [0] * n
        depth = [0] * n

        stack = [0]
        visited = [False] * n
        visited[0] = True
        while stack:
            u = stack.pop()
            for v, w in adj[u]:
                if not visited[v]:
                    visited[v] = True
                    depth[v] = depth[u] + 1
                    dist[v] = dist[u] + w
                    up[v][0] = u
                    for i in range(1, LOG):
                        if up[v][i-1] != -1:
                            up[v][i] = up[up[v][i-1]][i-1]
                        else:
                            up[v][i] = -1
                    stack.append(v)

        def get_lca(u: int, v: int) -> int:
            if depth[u] < depth[v]:
                u, v = v, u
            diff = depth[u] - depth[v]
            for i in range(LOG):
                if (diff >> i) & 1:
                    u = up[u][i]
            if u == v:
                return u
            for i in range(LOG-1, -1, -1):
                if up[u][i] != up[v][i]:
                    u = up[u][i]
                    v = up[v][i]
            return up[u][0]

        res = []
        for u, v in queries:
            if u == v:
                res.append(u)
                continue
            lca = get_lca(u, v)
            W = dist[u] + dist[v] - 2 * dist[lca]
            target = (W + 1) // 2
            dist_u_lca = dist[u] - dist[lca]

            if target <= dist_u_lca:
                cur = u
                d = 0
                for i in range(LOG-1, -1, -1):
                    if up[cur][i] != -1:
                        w = dist[cur] - dist[up[cur][i]]
                        if d + w < target:
                            d += w
                            cur = up[cur][i]
                res.append(up[cur][0])
            else:
                needed_from_lca = target - dist_u_lca
                max_from_v = (dist[v] - dist[lca]) - needed_from_lca
                cur = v
                d = 0
                for i in range(LOG-1, -1, -1):
                    if up[cur][i] != -1:
                        w = dist[cur] - dist[up[cur][i]]
                        if d + w <= max_from_v:
                            d += w
                            cur = up[cur][i]
                res.append(cur)
        return res
