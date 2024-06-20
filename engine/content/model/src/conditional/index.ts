import { Expression } from '../expression';

export interface Conditionally<V> {
    condition: Expression;
    value: V;
    next: Conditional<V>;
}

export type Conditional<V> = V | Conditionally<V>;
