import { Template } from '../template';

export interface Location {
    name: string;
    label: Template;
    description?: Template;
    body?: Template;
}
