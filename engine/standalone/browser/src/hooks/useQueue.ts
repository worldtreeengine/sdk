import React from 'react';

interface QueueEntry<T> {
    element: T;
    delay: number;
}

export function useQueue<T>(onPop: (element: T) => void): {
    push(element: T, delay?: number): void;
} {
    const queue: QueueEntry<T>[] = React.useMemo(() => [], []);

    const onPopRef = React.useRef(onPop);
    onPopRef.current = onPop;

    const pop = React.useCallback(() => {
        if (queue.length > 0) {
            const next = queue[0];
            try {
                onPopRef.current(next.element);
            } catch {
                // NOP
            }

            setTimeout(() => {
                queue.shift();
                pop();
            }, next.delay);
        }
    }, [ queue ]);

    const push = React.useCallback((element: T, delay?: number) => {
        queue.push({ element, delay: delay || 0 });
        if (queue.length === 1) {
            pop();
        }
    }, [ queue, pop ]);

    return { push };
}
