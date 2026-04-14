function cancellable<T>(generator: Generator<Promise<any>, T, unknown>): [() => void, Promise<T>] {
    let cancelled = false;
    let cancelFn!: () => void;

    const promise = new Promise<T>((resolve, reject) => {
        cancelFn = () => {
            cancelled = true;
            step(undefined, "Cancelled", true);
        };

        function step(value: any, error?: any, isError = false) {
            try {
                const result = isError ? generator.throw(error) : generator.next(value);
                if (result.done) {
                    resolve(result.value as T);
                } else {
                    (result.value as Promise<any>).then(
                        (val: any) => { if (!cancelled) step(val); },
                        (err: any) => step(undefined, err, true)
                    );
                }
            } catch (e) {
                reject(e);
            }
        }

        step(undefined);
    });

    return [cancelFn, promise];
}