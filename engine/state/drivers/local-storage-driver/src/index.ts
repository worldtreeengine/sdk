import { InMemoryStateDriver, State } from '@worldtreeengine/state.drivers.in-memory-driver';

export class LocalStorageStateDriver extends InMemoryStateDriver {
    private readonly key: string;
    readonly isNew: boolean;

    constructor(key: string) {
        const initialState: State = {
            qualities: {},
        };

        const storedState = window.localStorage.getItem(key);
        if (storedState) {
            try {
                const parsedState = JSON.parse(storedState);
                if (parsedState && typeof parsedState === 'object') {
                    if ('location' in parsedState && typeof parsedState.location === 'string') {
                        initialState.location = parsedState.location;
                    }

                    if ('storylet' in parsedState && typeof parsedState.storylet === 'string') {
                        initialState.storylet = parsedState.storylet;
                    }

                    if ('qualities' in parsedState && typeof parsedState.qualities === 'object') {
                        for (const key in parsedState.qualities) {
                            const value = parsedState.qualities[key];
                            if (value && typeof value === 'number' && value > 0) {
                                initialState.qualities[key] = value;
                            }
                        }
                    }
                }
            } catch (e) {
                // NOP
            }
        }
        super(initialState);
        this.key = key;
        this.isNew = !storedState;
    }

    protected async commit(...parameters: Parameters<InMemoryStateDriver['commit']>): Promise<void> {
        await super.commit(...parameters);

        const state = this.getState();
        const stateToStore: {
            location?: string;
            storylet?: string;
            qualities: Record<string, number>;
        } = {
            qualities: {},
        };

        if (state.location) {
            stateToStore.location = state.location;
        }

        if (state.storylet) {
            stateToStore.storylet = state.storylet;
        }

        for (const key in state.qualities) {
            let value = state.qualities[key];
            if (value > 0) {
                stateToStore.qualities[key] = value;
            }
        }

        window.localStorage.setItem(this.key, JSON.stringify(stateToStore));
    }
}
