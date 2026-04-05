type Fn = (...params: any) => any

function memoize(fn: Fn): Fn {
    const cache = new Map();
    
    return function(...args: any[]) {
        let node: any = cache;
        for (const arg of args) {
            if (!node.has(arg)) {
                node.set(arg, new Map());
            }
            node = node.get(arg);
        }
        if (!node.has('result')) {
            node.set('result', fn(...args));
        }
        return node.get('result');
    }
}
