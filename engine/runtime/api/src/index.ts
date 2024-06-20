import { State } from '@worldtreeengine/state.api';
import { QualityStyle, Text } from '@worldtreeengine/content.model';

export interface Runtime {
    begin(state: State): Promise<RuntimeSession>;
}

export interface RuntimeSessionState {
    location?: {
        label: Text;
        description?: Text;
    };
    storylet?: {
        label?: Text;
    };
    body?: Text;
    assignments?: Assignment[];
    choices?: Choice[];
    continue?: true | Text;
    prompt?: Text;
}

export interface Assignment {
    results?: AssignmentResult[];
    description?: Text;
}

export interface AssignmentResult {
    operation: 'set' | 'unset' | 'increment' | 'decrement';
    label: Text;
    value?: number | Text;
    description?: Text;
    style?: QualityStyle;
}

export interface Choice {
    id: number;
    label: Text;
    description?: Text;
    icon?: string;
}

export interface RuntimeSession {
    continue(): Promise<RuntimeSessionState>;
    choose(id: number): Promise<RuntimeSessionState>;
    reset(): Promise<void>;
}
