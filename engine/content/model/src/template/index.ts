import { Expression } from '../expression';

export interface BoldTemplateNode {
    b: Template;
}

export interface ItalicTemplateNode {
    i: Template;
}

export interface AnchorTemplateNode {
    a: Template;
    href: string;
}

export interface ConditionalTemplateNode {
    condition: Expression;
    value: Template;
    next?: Template;
}

export type TemplateNode = BoldTemplateNode | ItalicTemplateNode | AnchorTemplateNode | ConditionalTemplateNode | string;

export type Template = TemplateNode[];
