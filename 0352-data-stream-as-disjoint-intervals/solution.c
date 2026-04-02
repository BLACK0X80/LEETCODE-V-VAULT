typedef struct {
    bool seen[10001];
} SummaryRanges;

SummaryRanges* summaryRangesCreate() {
    SummaryRanges* s = calloc(1, sizeof(SummaryRanges));
    return s;
}

void summaryRangesAddNum(SummaryRanges* obj, int val) {
    obj->seen[val] = true;
}

int** summaryRangesGetIntervals(SummaryRanges* obj, int* retSize, int** retColSize) {
    int** res = malloc(10001 * sizeof(int*));
    *retColSize = malloc(10001 * sizeof(int));
    *retSize = 0;
    int i = 0;
    while (i <= 10000) {
        if (obj->seen[i]) {
            int start = i;
            while (i <= 10000 && obj->seen[i]) i++;
            res[*retSize] = malloc(2 * sizeof(int));
            res[*retSize][0] = start;
            res[*retSize][1] = i - 1;
            (*retColSize)[*retSize] = 2;
            (*retSize)++;
        } else i++;
    }
    return res;
}

void summaryRangesFree(SummaryRanges* obj) {
    free(obj);
}
