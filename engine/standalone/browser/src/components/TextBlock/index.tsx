import React from 'react';
import type { Text } from '@worldtreeengine/content.model';
import { InlineTextSpan } from '../InlineTextSpan';

export function TextBlock({ children }: {
    children: Text
}) {
    const nodes = React.useMemo(() => {
        return children.map((node, i) => {
            if (typeof node === 'object' && 'p' in node) {
                return <p key={i}><InlineTextSpan>{node.p}</InlineTextSpan></p>
            } else {
                return <InlineTextSpan key={i}>{[node]}</InlineTextSpan>
            }
        });
    }, [ children ]);

    return nodes;
}
