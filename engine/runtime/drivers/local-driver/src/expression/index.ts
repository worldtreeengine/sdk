import { Transaction } from '@worldtreeengine/state.api';
import { Expression, Model } from '@worldtreeengine/content.model';

export async function evaluateNumeric(expression: Expression, content: Model, transaction: Transaction): Promise<number> {
    if (typeof expression === 'number') {
        return expression;
    }

    if (typeof expression === 'string') {
        for (const quality of content.qualities) {
            if (quality.name === expression) {
                break;
            }

            if (quality.values) {
                for (let value = 0; value < quality.values.length; value++) {
                    if (quality.values[value].name === expression) {
                        return value + 1;
                    }
                }
            }
        }
        return transaction.get(expression);
    }

    if (expression.length < 2) {
        console.warn('Insufficient operands');
        return 0;
    }

    const operator = expression[0];
    const operands = expression.slice(1);

    switch (operator) {
        case 'plus':
            return evaluatePlus(operands, content, transaction);
        case 'multiply':
            return evaluateMultiply(operands, content, transaction);
        case 'minus':
            return evaluateMinus(operands, content, transaction);
        case 'divide':
            return evaluateDivide(operands, content, transaction);
        case 'equal':
            return await evaluateEqual(operands, content, transaction) ? 1 : 0;
        case 'notEqual':
            return await evaluateEqual(operands, content, transaction) ? 0 : 1;
        case 'greaterThan':
            return await evaluateGreaterThan(operands, content, transaction) ? 1 : 0;
        case 'greaterThanOrEqual':
            return await evaluateGreaterThanOrEqual(operands, content, transaction) ? 1 : 0;
        case 'lessThan':
            return await evaluateGreaterThanOrEqual(operands, content, transaction) ? 0 : 1;
        case 'lessThanOrEqual':
            return await evaluateGreaterThan(operands, content, transaction) ? 0 : 1;
        case 'and':
            return await evaluateAnd(operands, content, transaction) ? 1 : 0;
        case 'or':
            return await evaluateOr(operands, content, transaction) ? 1 : 0;
        case 'not':
            return await evaluateAnd(operands, content, transaction) ? 0 : 1;
        case 'maximum':
            return evaluateMaximum(operands, content, transaction);
        case 'minimum':
            return evaluateMinimum(operands, content, transaction);
        case 'random':
            return evaluateRandom(operands, content, transaction);
        case 'then':
            return evaluateNumeric(await evaluateLogical(operands[0], content, transaction) ? operands[0] : operands[1], content, transaction);
        case 'in':
            return await transaction.getLocation() === operands[0] ? 1 : 0;
    }

    console.warn('Unrecognized operator:', operator);
    return 0;
}

export async function evaluateLogical(expression: Expression, content: Model, transaction: Transaction): Promise<boolean> {
    if (typeof expression === 'number') {
        return expression > 0;
    }

    if (typeof expression === 'string') {
        for (const quality of content.qualities) {
            if (quality.name === expression) {
                break;
            }

            if (quality.values) {
                for (let value = 0; value < quality.values.length; value++) {
                    if (quality.values[value].name === expression) {
                        if (quality.exclusive) {
                            return await transaction.get(quality.name) === value + 1;
                        } else {
                            return await transaction.get(quality.name) >= value + 1;
                        }
                    }
                }
            }
        }
        return await transaction.get(expression) > 0;
    }

    if (expression.length < 2) {
        console.warn('Insufficient operands');
        return false;
    }

    const operator = expression[0];
    const operands = expression.slice(1);

    switch (operator) {
        case 'plus':
            return await evaluatePlus(operands, content, transaction) > 0;
        case 'multiply':
            return await evaluateMultiply(operands, content, transaction) > 0;
        case 'minus':
            return await evaluateMinus(operands, content, transaction) > 0;
        case 'divide':
            return await evaluateDivide(operands, content, transaction) > 0;
        case 'equal':
            return evaluateEqual(operands, content, transaction);
        case 'notEqual':
            return !await evaluateEqual(operands, content, transaction);
        case 'greaterThan':
            return evaluateGreaterThan(operands, content, transaction);
        case 'greaterThanOrEqual':
            return evaluateGreaterThanOrEqual(operands, content, transaction);
        case 'lessThan':
            return !await evaluateGreaterThanOrEqual(operands, content, transaction);
        case 'lessThanOrEqual':
            return !await evaluateGreaterThan(operands, content, transaction);
        case 'and':
            return evaluateAnd(operands, content, transaction);
        case 'or':
            return evaluateOr(operands, content, transaction);
        case 'not':
            return !await evaluateAnd(operands, content, transaction);
        case 'maximum':
            return await evaluateMaximum(operands, content, transaction) > 0;
        case 'minimum':
            return await evaluateMinimum(operands, content, transaction) > 0;
        case 'random':
            return await evaluateRandom(operands, content, transaction) > 0;
        case 'then':
            return evaluateLogical(await evaluateLogical(operands[0], content, transaction) ? operands[0] : operands[1], content, transaction);
        case 'in':
            return await transaction.getLocation() === operands[0];
    }

    console.warn('Unrecognized operator:', operator);
    return false;
}

declare const console: {
    warn(...messages: unknown[]): void;
};

async function evaluatePlus(operands: Expression[], content: Model, transaction: Transaction) {
    let sum = 0;
    for (const operand of operands) {
        sum += await evaluateNumeric(operand, content, transaction);
    }
    return sum;
}

async function evaluateMultiply(operands: Expression[], content: Model, transaction: Transaction) {
    let product = 0;
    for (const operand of operands) {
        product *= await evaluateNumeric(operand, content, transaction);
    }
    return product;
}

async function evaluateMinus(operands: Expression[], content: Model, transaction: Transaction) {
    let difference = await evaluateNumeric(operands[0], content, transaction);
    for (const operand of operands.slice(1)) {
        difference -= await evaluateNumeric(operand, content, transaction);
    }
    return Math.max(difference, 0);
}

async function evaluateDivide(operands: Expression[], content: Model, transaction: Transaction) {
    let quotient = await evaluateNumeric(operands[0], content, transaction);
    for (const operand of operands.slice(1)) {
        let divisor = await evaluateNumeric(operand, content, transaction);
        if (divisor === 0) {
            return 0;
        }
        quotient /= divisor;
    }
    return Math.trunc(quotient);
}

async function evaluateEqual(operands: Expression[], content: Model, transaction: Transaction) {
    const left = await evaluateNumeric(operands[0], content, transaction);
    const right = await evaluateNumeric(operands[1], content, transaction);
    return left === right;
}

async function evaluateGreaterThan(operands: Expression[], content: Model, transaction: Transaction) {
    const left = await evaluateNumeric(operands[0], content, transaction);
    const right = await evaluateNumeric(operands[1], content, transaction);
    return left > right;
}

async function evaluateGreaterThanOrEqual(operands: Expression[], content: Model, transaction: Transaction) {
    const left = await evaluateNumeric(operands[0], content, transaction);
    const right = await evaluateNumeric(operands[1], content, transaction);
    return left >= right;
}

async function evaluateAnd(operands: Expression[], content: Model, transaction: Transaction) {
    for (const operand of operands) {
        if (!await evaluateLogical(operand, content, transaction)) {
            return false;
        }
    }
    return true;
}

async function evaluateOr(operands: Expression[], content: Model, transaction: Transaction) {
    for (const operand of operands) {
        if (await evaluateLogical(operand, content, transaction)) {
            return true;
        }
    }
    return false;
}

async function evaluateMinimum(operands: Expression[], content: Model, transaction: Transaction) {
    let minimum = -1;
    for (const operand in operands) {
        const value = await evaluateNumeric(operand, content, transaction);
        if (value < minimum || minimum === -1) {
            minimum = value;
        }
    }
    return Math.max(0, minimum);
}

async function evaluateMaximum(operands: Expression[], content: Model, transaction: Transaction) {
    let maximum = 0;
    for (const operand in operands) {
        const value = await evaluateNumeric(operand, content, transaction);
        if (value > maximum) {
            maximum = value;
        }
    }
    return maximum;
}

async function evaluateRandom(operands: Expression[], content: Model, transaction: Transaction) {
    let minimum = -1;
    let maximum = 0;
    for (const operand in operands) {
        const value = await evaluateNumeric(operand, content, transaction);
        if (value < minimum || minimum === -1) {
            minimum = value;
        }
        if (value > maximum) {
            maximum = value;
        }
    }
    return Math.max(0, Math.trunc(Math.random() * (maximum - minimum)) + minimum);
}
