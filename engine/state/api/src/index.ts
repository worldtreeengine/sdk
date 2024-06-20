export type TransactionRunner<T> = (transaction: Transaction) => Promise<T>;

export interface State {
    withTransaction<T>(runner: TransactionRunner<T>): Promise<T>;
}

export interface Transaction {
    get(name: string): Promise<number>;
    getLocation(): Promise<string | undefined>;
    set(name: string, value: number): Promise<Effect | undefined>;
    unset(name: string, value: number): Promise<Effect | undefined>;
    increment(name: string, step: number): Promise<Effect | undefined>;
    decrement(name: string, step: number): Promise<Effect | undefined>;
    setLocation(name: string): Promise<void>;
    unsetLocation(): Promise<void>;
    getStorylet(): Promise<string | undefined>;
    setStorylet(id: string): Promise<void>;
    commitStorylet(): Promise<void>;
    clear(): Promise<void>;
}

export interface Effect {
    before: number;
    after: number;
}

export abstract class TransactionBase implements Transaction {
    async set(name: string, value: number): Promise<Effect | undefined> {
        const oldValue = await this.get(name);
        if (oldValue < value) {
            await this.update(name, value);
            return {
                before: oldValue,
                after: value,
            };
        }
    }

    async unset(name: string, value: number): Promise<Effect | undefined> {
        if (value > 0) {
            const oldValue = await this.get(name);
            if (oldValue > value) {
                await this.update(name, value);
                return {
                    before: oldValue,
                    after: value,
                };
            }
        }
    }

    async increment(name: string, step: number): Promise<Effect | undefined> {
        const oldValue = await this.get(name);
        const newValue = oldValue + step;
        if (newValue > oldValue) {
            await this.update(name, newValue);
            return {
                before: oldValue,
                after: newValue,
            };
        }
    }

    async decrement(name: string, step: number): Promise<Effect | undefined> {
        const oldValue = await this.get(name);
        const newValue = Math.max(0, oldValue - step);
        if (newValue < oldValue) {
            await this.update(name, newValue);
            return {
                before: oldValue,
                after: newValue,
            };
        }
    }

    protected abstract update(name: string, value: number): Promise<void>;

    abstract clear(): Promise<void>;
    abstract get(name: string): Promise<number>;
    abstract getLocation(): Promise<string | undefined>;
    abstract setLocation(name: string): Promise<void>;
    abstract unsetLocation(): Promise<void>;
    abstract getStorylet(): Promise<string | undefined>;
    abstract setStorylet(name: string): Promise<void>;
    abstract commitStorylet(): Promise<void>;
}

export * from './serialized';
