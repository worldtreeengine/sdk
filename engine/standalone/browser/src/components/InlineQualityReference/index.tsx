import React from 'react';
import { InlineTextSpan } from '../InlineTextSpan';
import type { Text } from '@worldtreeengine/content.model';

export function InlineQualityReference({ name, children }: {
    name: string;
    children: Text;
}) {
    return <span className="inline-quality-reference" data-quality-id={name}><InlineTextSpan>{children}</InlineTextSpan></span>
}
