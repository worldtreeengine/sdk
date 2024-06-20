import React from 'react';
import type { Text } from '@worldtreeengine/content.model';
import { NonInteractiveTextSpan } from '../NonInteractiveTextSpan';

export function InlineTextSpan({ capitalize, children }: {
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
                return <React.Fragment key={i}>{space()}<i><InlineTextSpan capitalize={capitalize && i === 0}>{node.i}</InlineTextSpan></i></React.Fragment>
            }

            if ('b' in node) {
                return <React.Fragment key={i}>{space()}<b><InlineTextSpan capitalize={capitalize && i === 0}>{node.b}</InlineTextSpan></b></React.Fragment>
            }

            if ('a' in node) {
                const url = new URL(node.href, document.location.toString());
                const target = url.origin === document.location.origin ? undefined : '_blank';

                return <React.Fragment key={i}>{space()}<a key={i} href={node.href} target={target}><NonInteractiveTextSpan capitalize={capitalize && i === 0}>{node.a}</NonInteractiveTextSpan></a></React.Fragment>
            }

            if ('p' in node) {
                return <React.Fragment key={i}>{space(true)}<InlineTextSpan capitalize={capitalize && i === 0}>{node.p}</InlineTextSpan></React.Fragment>
            }
        });
    }, [ children, capitalize ]);

    return nodes;
}
