import { State, Transaction, TransactionRunner } from './index';

interface QueuedRunner<T> {
    runner: TransactionRunner<T>;
    resolve: Resolve<T>;
    reject: Reject;
}

type Resolve<T> = (value: T) => void;
type Reject = (error: unknown) => void;

export abstract class SerializedState<T extends Transaction> implements State {
    private queue: QueuedRunner<any>[] = [];

    withTransaction<T>(runner: (transaction: Transaction) => Promise<T>): Promise<T> {
        return new Promise<T>((resolve, reject) => {
           this.queue.push({
               runner,
               resolve,
               reject,
           });
           if (this.queue.length === 1) {
               this.runQueue();
           }
        });
    }

    private runQueue() {
        if (this.queue.length > 0) {
            const queued = this.queue[0];
            this.begin()
                .then((transaction) => {
                    queued.runner(transaction)
                        .then((value) => {
                            this.commit(transaction).then(() => queued.resolve(value)).catch((error) => {
                                this.rollback(transaction).finally(() => queued.reject(error));
                            });
                        })
                        .catch((error) => {
                            this.rollback(transaction).finally(() => queued.reject(error));
                        });
                })
                .catch((error) => {
                    queued.reject(error);
                })
                .finally(() => {
                    this.queue.shift();
                    this.runQueue();
                });
        }
    }

    protected abstract begin(): Promise<T>;
    protected abstract commit(transaction: T): Promise<void>;
    protected abstract rollback(transaction: T): Promise<void>;
}
