#include <string.h>
#include <stdlib.h>

int overlap[12][12];
int dp[1<<12][12];
int par[1<<12][12];

int ov(char *a, char *b) {
    int la = strlen(a), lb = strlen(b);
    for (int i = la; i >= 0; i--)
        if (strncmp(a + la - i, b, i) == 0) return i;
    return 0;
}

char* shortestSuperstring(char** w, int n) {
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            overlap[i][j] = ov(w[i], w[j]);

    int full = 1 << n;
    for (int s = 0; s < full; s++)
        for (int i = 0; i < n; i++)
            dp[s][i] = -1;

    for (int i = 0; i < n; i++) { dp[1<<i][i] = strlen(w[i]); par[1<<i][i] = -1; }

    for (int s = 1; s < full; s++) {
        for (int i = 0; i < n; i++) {
            if (!(s & (1<<i)) || dp[s][i] < 0) continue;
            for (int j = 0; j < n; j++) {
                if (s & (1<<j)) continue;
                int ns = s | (1<<j);
                int val = dp[s][i] + (int)strlen(w[j]) - overlap[i][j];
                if (dp[ns][j] < 0 || val < dp[ns][j]) {
                    dp[ns][j] = val;
                    par[ns][j] = i;
                }
            }
        }
    }

    int last = 0, best = -1;
    for (int i = 0; i < n; i++)
        if (best < 0 || dp[full-1][i] < best) { best = dp[full-1][i]; last = i; }

    int path[12], ps = 0, s = full - 1, cur = last;
    while (cur != -1) {
        path[ps++] = cur;
        int prev = par[s][cur];
        s ^= (1<<cur);
        cur = prev;
    }

    char *res = malloc(250);
    res[0] = 0;
    for (int i = ps-1; i >= 0; i--) {
        int wi = path[i];
        if (i == ps-1) { strcat(res, w[wi]); continue; }
        int prev = path[i+1];
        strcat(res, w[wi] + overlap[prev][wi]);
    }
    return res;
}
