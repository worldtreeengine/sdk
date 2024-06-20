import { Expression } from '../expression';
import { Template } from '../template';
import { Conditional } from '../conditional';

export interface Storylet {
    name: string;
    condition?: Expression;
    label?: Template;
    description?: Template;
    icon?: Conditional<string>;
    body?: Template;
    navigation?: Conditional<string>;
    assignments?: AssignmentGroup[];
    choices?: Choices;
}

export interface AssignmentGroup {
    assignments: Assignment[];
    description?: Template;
}

export interface Assignment {
    condition?: Expression;
    subject: string;
    operation: 'set' | 'unset' | 'increment' | 'decrement';
    operand: Expression;
    hidden?: Expression;
}

export interface Choices {
    prompt?: Template;
    groups: ChoiceGroup[];
}

export interface ChoiceGroup {
    limit?: Expression;
    shuffle?: Expression;
    choices: Choice[];
}

export interface Choice {
    condition?: Expression;
    label: Template;
    description?: Template;
    icon?: Conditional<string>;
    body?: Template;
    navigation?: Conditional<string>;
    assignments?: AssignmentGroup[];
}
