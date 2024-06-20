import React from 'react';
import type { Text } from '@worldtreeengine/content.model';

export function NonInteractiveTextSpan({ capitalize, children }: {
    capitalize?: boolean;
    children: Text,
}) {
    const nodes: React.ReactNode = React.useMemo(() => {
        let needsSpace = false;
        const space = (needsSpaceAfter?: boolean) => {
            const result = needsSpace ? ' ' : '';
            needsSpace = !!needsSpaceAfter;
            return result;
        };

        return children.map((node, i) => {
            if (typeof node === 'string') {
                if (capitalize && i === 0) {
                    return space() + node.slice(0, 1).toUpperCase() + node.slice(1);
                } else {
                    return space() + node;
                }
            }

            if ('i' in node) {
                return <React.Fragment key={i}>{space()}<i><NonInteractiveTextSpan capitalize={capitalize && i === 0}>{node.i}</NonInteractiveTextSpan></i></React.Fragment>
            }

            if ('b' in node) {
                return <React.Fragment key={i}>{space()}<b><NonInteractiveTextSpan capitalize={capitalize && i === 0}>{node.b}</NonInteractiveTextSpan></b></React.Fragment>
            }

            if ('a' in node) {
                return <React.Fragment key={i}>{space()}<NonInteractiveTextSpan capitalize={capitalize && i === 0}>{node.a}</NonInteractiveTextSpan></React.Fragment>
            }

            if ('p' in node) {
                return <React.Fragment key={i}>{space(true)}<NonInteractiveTextSpan capitalize={capitalize && i === 0}>{node.p}</NonInteractiveTextSpan></React.Fragment>
            }
        });
    }, [ children, capitalize ]);

    return nodes;
}
