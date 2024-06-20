import { Model, Template, Text } from '@worldtreeengine/content.model';
import { Transaction } from '@worldtreeengine/state.api';
import { evaluateLogical } from '../expression';

export async function evaluateTemplate(template: Template, content: Model, transaction: Transaction): Promise<Text> {
    let flat = await evaluateTemplateFlat(template, content, transaction);

    let p: Text = [];
    const result: Text = [{ p }];

    for (const node of flat) {
        if (node === '\n') {
            p = [];
            result.push({ p });
        } else {
            p.push(node);
        }
    }

    return result;
}

async function evaluateTemplateFlat(template: Template, content: Model, transaction: Transaction): Promise<Text> {
    let result: Text = [];

    for (const node of template) {
        if (typeof node === 'string') {
            result.push(node);
        } else if ('i' in node) {
            result.push({ i: await evaluateTemplateFlat(node.i, content, transaction) });
        } else if ('b' in node) {
            result.push({ b: await evaluateTemplateFlat(node.b, content, transaction) });
        } else if ('a' in node) {
            result.push({
                a: await evaluateTemplateFlat(node.a, content, transaction),
                href: node.href,
            });
        } else if ('condition' in node) {
            if (await evaluateLogical(node.condition, content, transaction)) {
                result.push(...await evaluateTemplateFlat(node.value, content, transaction));
            } else if (node.next) {
                result.push(...await evaluateTemplateFlat(node.next, content, transaction));
            }
        }
    }

    return result;
}
