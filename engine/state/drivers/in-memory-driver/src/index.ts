import { SerializedState, TransactionBase } from '@worldtreeengine/state.api';

export interface State {
    location?: string;
    storylet?: string;
    qualities: Record<string, number>;
}

export class InMemoryStateDriver extends SerializedState<InMemoryTransaction> {
    private state: State;

    constructor(initialState?: State) {
        super();
        this.state = initialState || {
            qualities: {},
        };
    }

    getState(): State {
        return {
            ...this.state,
            qualities: {
                ...this.state.qualities,
            },
        };
    }

    protected async begin(): Promise<InMemoryTransaction> {
        return new InMemoryTransaction({
            ...this.state,
            qualities: {
                ...this.state.qualities,
            },
        });
    }

    protected async commit(transaction: InMemoryTransaction): Promise<void> {
        this.state = transaction.getState();
    }

    protected async rollback(transaction: InMemoryTransaction): Promise<void> {
        // NOP
    }

}

class InMemoryTransaction extends TransactionBase {
    private state: State;

    constructor(initialState: State) {
        super();
        this.state = initialState;
    }

    getState(): State {
        return {
            ...this.state,
            qualities: {
                ...this.state.qualities,
            },
        };
    }

    async clear(): Promise<void> {
        this.state = {
            qualities: {},
        };
    }

    async get(id: string): Promise<number> {
        return this.state.qualities[id] || 0;
    }

    async getLocation(): Promise<string | undefined> {
        return this.state.location;
    }

    protected async update(name: string, value: number): Promise<void> {
        if (value === 0) {
            delete this.state.qualities[name];
        } else {
            this.state.qualities[name] = value;
        }
    }

    async setLocation(name: string): Promise<void> {
        this.state.location = name;
        return this.update(name, await this.get(name) + 1);
    }

    async unsetLocation(): Promise<void> {
        delete this.state.location;
    }

    async commitStorylet(): Promise<void> {
        if (this.state.storylet) {
            await this.increment(this.state.storylet, 1);
            delete this.state.storylet;
        }
    }

    async getStorylet(): Promise<string | undefined> {
        return this.state.storylet;
    }

    async setStorylet(id: string): Promise<void> {
        await this.commitStorylet();
        this.state.storylet = id;
    }
}
