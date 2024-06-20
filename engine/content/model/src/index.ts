import { Location } from './location';
import { Quality } from './quality';
import { Storylet } from './storylet';
import { Text } from './text';

export interface Model {
    meta: {
        title?: Text;
        description?: Text;
        credits?: Text[];
    };
    qualities: Quality[];
    locations: Location[];
    storylets: Storylet[];
}

export * from './conditional';
export * from './expression';
export * from './location';
export * from './quality';
export * from './storylet';
export * from './template'
export * from './text';
