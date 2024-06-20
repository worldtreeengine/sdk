import { Template } from '../template';
import { Conditional } from '../conditional';

export interface Quality {
    name: string;
    hidden?: boolean;
    label?: Template;
    singularLabel?: Template;
    pluralLabel?: Template;
    description?: Template;
    icon?: Conditional<string>;
    style?: QualityStyle;
    exclusive?: boolean;
    values?: QualityValue[];
}

export interface QualityValue {
    name: string;
    label?: Template;
    description?: Template;
    icon?: Conditional<string>;
}

export interface QualityStyle {
    currency: boolean;
    personal: boolean;
    plural: boolean;
    possessive: boolean;
    uncounted: boolean;
}
