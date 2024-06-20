import { Conditional, Model } from '@worldtreeengine/content.model';
import { Transaction } from '@worldtreeengine/state.api';
import { evaluateLogical } from '../expression';

export async function evaluateConditional<V>(conditional: Conditional<V>, content: Model, transaction: Transaction): Promise<V> {
    if (conditional && typeof conditional === 'object') {
        if ('condition' in conditional) {
            if (await evaluateLogical(conditional.condition, content, transaction)) {
                return conditional.value;
            } else {
                return evaluateConditional(conditional.next, content, transaction);
            }
        }
    }

    return conditional;
}
