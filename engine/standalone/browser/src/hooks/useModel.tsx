import React from 'react';
import { Model } from '@worldtreeengine/content.model';

const ModelContext = React.createContext<Model | null>(null);

export function useModel() {
    const model = React.useContext(ModelContext);
    if (!model) {
        throw new Error('useWorld called outside a WorldProvider');
    }
    return model;
}

export function ModelProvider ({ model, children }: {
    model: Model,
    children: React.ReactNode,
}) {
    return <ModelContext.Provider value={model}>{children}</ModelContext.Provider>;
}
