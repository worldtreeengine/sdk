import {
    Location,
    Model, Storylet, Template, Text, Choice as ChoiceModel, Choices, AssignmentGroup, Quality
} from '@worldtreeengine/content.model';
import {
    Runtime,
    RuntimeSession, RuntimeSessionState, Choice, Assignment, AssignmentResult
} from '@worldtreeengine/runtime.api';
import { Effect, State, Transaction } from '@worldtreeengine/state.api';
import { evaluateTemplate } from './template';
import { evaluateLogical, evaluateNumeric } from './expression';
import { evaluateConditional } from './conditional';

declare const console: {
    log(...message: unknown[]): void;
    warn(...message:unknown[]): void;
    error(...message: unknown[]): void;
}

export class LocalRuntimeDriver implements Runtime {
    private readonly model: Model;

    constructor(world: Model) {
        this.model = world;
    }

    async begin(state: State): Promise<RuntimeSession> {
        return new LocalRuntimeSession(this.model, state);
    }
}

class LocalRuntimeSession implements RuntimeSession {
    private readonly model: Model;
    private readonly state: State;

    private location?: Location;
    private storylet?: Storylet;

    private readonly body: Text = [];
    private readonly assignments: Assignment[] = [];

    private readonly pendingAssignments: AssignmentGroup[] = [];
    private pendingNavigationDestination?: string;

    private readonly recentStorylets: number[] = [];

    private choices?: ChoiceModel[] = undefined;

    private started = false;

    private staleBody = false;
    private staleAssignments = false;

    constructor(model: Model, state: State) {
        this.model = model;
        this.state = state;
    }

    continue(): Promise<RuntimeSessionState> {
        return this.state.withTransaction(async (transaction): Promise<RuntimeSessionState> => {
            return this.execute(transaction);
        });
    }

    private async execute(transaction: Transaction): Promise<RuntimeSessionState> {
        if (!this.started) {
            await this.start(transaction);
            this.started = true;
        }

        if (!this.storylet) {
            await this.evaluateEligibleStorylets(transaction);
        }

        while (this.storylet) {
            if (this.storylet.body) {
                if (this.staleBody) {
                    this.body.splice(0);
                    this.staleBody = false;
                }
                if (this.staleAssignments) {
                    this.assignments.splice(0);
                    this.staleAssignments = false;
                }
                this.body.push(...await evaluateTemplate(this.storylet.body, this.model, transaction));
            }

            if (this.storylet.assignments) {
                this.pendingAssignments.push(...this.storylet.assignments);
            }

            if (this.storylet.navigation) {
                this.pendingNavigationDestination = await evaluateConditional(this.storylet.navigation, this.model, transaction);
            }

            if (this.storylet.choices) {
                const choices = await this.gatherChoices(this.storylet.choices, transaction);
                if (choices.length) {
                    this.choices = choices;
                    const evaluatedChoices: Choice[] = [];
                    for (let i = 0; i < choices.length; i++) {
                        const choice = choices[i];
                        const label = await evaluateTemplate(choice.label, this.model, transaction);
                        const description = choice.description && await evaluateTemplate(choice.description, this.model, transaction);
                        const icon = choice.icon && await evaluateConditional(choice.icon, this.model, transaction);
                        evaluatedChoices.push({
                            id: i,
                            label,
                            description,
                            icon,
                        });
                    }
                    return this.stateWithChoices(evaluatedChoices, this.storylet.choices.prompt, transaction);
                }
            }

            const assignments = await this.commit(transaction);
            if (assignments.length) {
                if (this.staleAssignments) {
                    this.assignments.splice(0);
                    this.staleAssignments = false;
                }
                this.assignments.push(...assignments);
            }
        }

        const choices = await this.evaluateAvailableStorylets(transaction);

        if (this.location?.body) {
            if (this.staleBody) {
                this.body.splice(0);
                this.staleBody = false;
            }
            if (this.staleAssignments) {
                this.assignments.splice(0);
                this.staleAssignments = false;
            }
            if (this.body.length === 0) {
                this.body.push(...await evaluateTemplate(this.location?.body, this.model, transaction));
            }
        }

        if (choices.length > 0) {
            return this.stateWithChoices(choices, undefined, transaction);
        } else {
            return this.stateWithContinue(true, undefined, transaction);
        }
    }

    private async executeChoice(choice: ChoiceModel, transaction: Transaction): Promise<RuntimeSessionState> {
        if (choice.body) {
            if (this.staleBody) {
                this.body.splice(0);
                this.staleBody = false;
            }
            if (this.staleAssignments) {
                this.assignments.splice(0);
                this.staleAssignments = false;
            }
            this.body.splice(0);
            this.assignments.splice(0);
            this.body.push(...await evaluateTemplate(choice.body, this.model, transaction));
        }

        if (choice.assignments) {
            this.pendingAssignments.push(...choice.assignments);
        }

        if (choice.navigation) {
            this.pendingNavigationDestination = await evaluateConditional(choice.navigation, this.model, transaction);
        }

        const assignments = await this.commit(transaction);
        if (assignments.length) {
            if (this.staleAssignments) {
                this.assignments.splice(0);
                this.staleAssignments = false;
            }
            this.assignments.push(...assignments);
        }

        return this.execute(transaction);
    }

    private async renderAssignmentResult(quality: Quality, effect: Effect, transaction: Transaction): Promise<AssignmentResult | undefined> {
        if (quality.hidden) {
            return undefined;
        }

        if (quality.values) {
            let qualityValue = quality.values[effect.after ? effect.after - 1 : effect.before - 1];
            if (!qualityValue) {
                return undefined;
            }

            let qualityValueLabel = qualityValue.label ? await evaluateTemplate(qualityValue.label, this.model, transaction)
                : [ quality.name ];
            let description = qualityValue.description ? await evaluateTemplate(qualityValue.description, this.model, transaction)
                : quality.description && await evaluateTemplate(quality.description, this.model, transaction);

            if (quality.style?.uncounted) {
                let label = qualityValueLabel;
                let value = effect.after ? 1 : 0;
                let operation: 'set' | 'unset' = effect.after ? 'set' : 'unset';
                return {
                    label,
                    value,
                    operation,
                    description,
                    style: quality.style,
                };
            } else {
                let label = quality.style?.plural && quality.pluralLabel ? await evaluateTemplate(quality.pluralLabel, this.model, transaction)
                    : quality.label ? await evaluateTemplate(quality.label, this.model, transaction)
                    : [ quality.name ];
                let value = qualityValueLabel;
                let operation: 'set' | 'unset' = effect.after ? 'set' : 'unset';
                return {
                    label,
                    value,
                    operation,
                    description,
                    style: quality.style,
                };
            }
        } else {
            let description = quality.description && await evaluateTemplate(quality.description, this.model, transaction);

            if (quality.style?.uncounted) {
                if (effect.before > 0 && effect.after > 0) {
                    return undefined;
                }

                let label = quality.style?.plural && quality.pluralLabel ? await evaluateTemplate(quality.pluralLabel, this.model, transaction) :
                    !quality.style?.plural && quality.singularLabel ? await evaluateTemplate(quality.singularLabel, this.model, transaction) :
                    quality.label ? await evaluateTemplate(quality.label, this.model, transaction) : [quality.name];
                let value = effect.after === 0 ? 0 : 1;
                let operation: 'set' | 'unset' = effect.after === 0 ? 'unset' : 'set';
                return {
                    label,
                    value,
                    operation,
                    description,
                    style: quality.style,
                };
            } else if (quality.style?.currency) {
                let label = effect.after === 1 && quality.singularLabel ? await evaluateTemplate(quality.singularLabel, this.model, transaction) :
                    effect.after !== 1 && quality.pluralLabel ? await evaluateTemplate(quality.pluralLabel, this.model, transaction) :
                    quality.label ? await evaluateTemplate(quality.label, this.model, transaction) : [ quality.name ];
                let value = effect.after;
                let operation: 'increment' | 'decrement' = effect.after > effect.before ? 'increment' : 'decrement';
                return {
                    label,
                    value,
                    operation,
                    description,
                    style: quality.style,
                }
            } else {
                let label = quality.style?.plural && quality.pluralLabel ? await evaluateTemplate(quality.pluralLabel, this.model, transaction) :
                    !quality.style?.plural && quality.singularLabel ? await evaluateTemplate(quality.singularLabel, this.model, transaction) :
                    quality.label ? await evaluateTemplate(quality.label, this.model, transaction) : [quality.name];
                let value = effect.after;
                let operation: 'increment' | 'decrement' = effect.after > effect.before ? 'increment' : 'decrement';
                return {
                    label,
                    value,
                    operation,
                    description,
                    style: quality.style,
                };
            }
        }
    }

    private async commit(transaction: Transaction): Promise<Assignment[]> {
        const assignments: Assignment[] = [];

        for (const group of this.pendingAssignments) {
            const results: AssignmentResult[] = [];
            for (const { condition, subject, operation, operand, hidden } of group.assignments) {
                if (condition === undefined || await evaluateLogical(condition, this.model, transaction)) {
                    let found = false;
                    for (const quality of this.model.qualities) {
                        if (quality.name === subject) {
                            found = true;
                            let effect: Effect | undefined = undefined;
                            let value = await evaluateNumeric(operand, this.model, transaction);
                            if (operation === 'set') {
                                effect = await transaction.set(subject, value);
                            } else if (operation === 'unset') {
                                effect = await transaction.unset(subject, value);
                            } else if (operation === 'increment') {
                                effect = await transaction.increment(subject, value);
                            } else if (operation === 'decrement') {
                                effect = await transaction.decrement(subject, value);
                            }

                            if (effect && (hidden === undefined || await evaluateLogical(hidden, this.model, transaction)) && !quality.hidden) {
                                const result = await this.renderAssignmentResult(quality, effect, transaction);
                                if (result) {
                                    results.push(result);
                                }
                            }

                            break;
                        } else if (quality.values) {
                            let value = 0;
                            for (let i = 0; i < quality.values.length; i++) {
                                if (quality.values[i].name === subject) {
                                    found = true;
                                    value = i;
                                    break;
                                }
                            }

                            if (found) {
                                let effect: Effect | undefined = undefined;
                                let evaluatedValue = await evaluateNumeric(operand, this.model, transaction);
                                if (operation === 'set') {
                                    if (evaluatedValue === 1) {
                                        if (quality.exclusive) {
                                            effect = await transaction.set(quality.name, value + 1) || await transaction.unset(quality.name, value + 1);
                                        } else {
                                            effect = await transaction.set(quality.name, value + 1);
                                        }
                                    } else {
                                        effect = await transaction.set(quality.name, evaluatedValue);
                                    }
                                } else if (operation === 'unset') {
                                    if (evaluatedValue === 0) {
                                        if (quality.exclusive) {
                                            effect = await transaction.unset(quality.name, 0);
                                        } else {
                                            effect = await transaction.unset(quality.name, value);
                                        }
                                    } else {
                                        effect = await transaction.unset(quality.name, evaluatedValue);
                                    }
                                } else if (operation === 'increment') {
                                    effect = await transaction.increment(quality.name, evaluatedValue);
                                } else if (operation === 'decrement') {
                                    effect = await transaction.decrement(quality.name, evaluatedValue);
                                }

                                if (effect && (hidden === undefined || !await evaluateLogical(hidden, this.model, transaction)) && !quality.hidden) {
                                    const result = await this.renderAssignmentResult(quality, effect, transaction);
                                    if (result) {
                                        results.push(result);
                                    }
                                }

                                break;
                            }
                        }
                    }

                    if (!found) {
                        let value = await evaluateNumeric(operand, this.model, transaction);
                        if (operation === 'set') {
                            await transaction.set(subject, value);
                        } else if (operation === 'unset') {
                            await transaction.unset(subject, value);
                        } else if (operation === 'increment') {
                            await transaction.increment(subject, value);
                        } else if (operation === 'decrement') {
                            await transaction.increment(subject, value);
                        }
                    }
                }
            }
            if (results.length > 0 || group.description) {
                this.assignments.push({
                    results: results.length > 0 ? results : undefined,
                    description: group.description && await evaluateTemplate(group.description, this.model, transaction),
                });
            }
        }

        this.pendingAssignments.splice(0);

        if (this.pendingNavigationDestination) {
            const location = this.model.locations.find((location) => location.name === this.pendingNavigationDestination);
            if (location) {
                this.location = location;
                await transaction.setLocation(location.name);
            }
        }

        this.pendingNavigationDestination = undefined;

        await transaction.commitStorylet();
        this.storylet = undefined;
        this.choices = undefined;

        await this.evaluateEligibleStorylets(transaction);

        return assignments;
    }

    private async gatherChoices(choices: Choices, transaction: Transaction): Promise<ChoiceModel[]> {
        const result: ChoiceModel[] = [];
        for (const group of choices.groups) {
            let shuffle = group.shuffle !== undefined && await evaluateLogical(group.shuffle, this.model, transaction);
            let limit = group.limit && await evaluateNumeric(group.limit, this.model, transaction);
            if (shuffle) {
                let choices = [...group.choices];
                for (let i = 0; i < choices.length; i++) {
                    const j = Math.trunc(Math.random() * choices.length);
                    const swap = choices[i];
                    choices[i] = choices[j];
                    choices[j] = swap;
                }
                result.push(...choices.slice(0, limit || undefined));
            } else {
                result.push(...group.choices.slice(0, limit || undefined));
            }
        }
        return result;
    }

    choose(id: number): Promise<RuntimeSessionState> {
        return this.state.withTransaction(async (transaction) => {
            if (this.storylet) {
                const choice = this.choices && this.choices[id];
                if (choice && (choice.condition === undefined || await evaluateLogical(choice.condition, this.model, transaction))) {
                    return this.executeChoice(choice, transaction);
                }
            } else {
                const storylet = this.model.storylets[id];
                if (storylet && (storylet.condition === undefined || await evaluateLogical(storylet.condition, this.model, transaction))) {
                    this.storylet = storylet;
                    await transaction.setStorylet(storylet.name);
                }
            }

            return this.execute(transaction);
        });
    }

    reset(): Promise<void> {
        return this.state.withTransaction(async (transaction) => {
            await transaction.clear();

            this.storylet = undefined;
            this.location = undefined;
            this.choices = undefined;

            this.body.splice(0);
            this.assignments.splice(0);

            this.pendingAssignments.splice(0);
            this.pendingNavigationDestination = undefined;

            this.staleBody = false;
            this.staleAssignments = false;

            this.recentStorylets.splice(0);
        });
    }

    private async start(transaction: Transaction): Promise<void> {
        const locationId = await transaction.getLocation();
        if (locationId) {
            const location = this.model.locations.find((location) => location.name === locationId);
            if (location) {
                this.location = location;
            }
        }

        const storyletId = await transaction.getStorylet();
        if (storyletId) {
            const storylet = this.model.storylets.find((storylet) => storylet.name === storyletId);
            if (storylet) {
                this.storylet = storylet;
            }
        }
    }

    private async evaluateEligibleStorylets(transaction: Transaction): Promise<void> {
        const results: number[] = [];

        for (let i = 0; i < this.model.storylets.length; i++) {
            const storylet = this.model.storylets[i];
            if (!storylet.label && (storylet.condition === undefined || await evaluateLogical(storylet.condition, this.model, transaction))) {
                results.push(i);
            }
        }

        if (results.length) {
            for (let i = this.recentStorylets.length - 1; i >= 0; i--) {
                if (!results.includes(this.recentStorylets[i])) {
                    this.recentStorylets.splice(i, 1);
                }
            }

            for (let i = results.length; i >= 0; i--) {
                if (this.recentStorylets.includes(results[i])) {
                    results.splice(i, 1);
                }
            }
        }

        if (results.length) {
            console.log(results);
            const index = results[Math.trunc(Math.random() * results.length)];
            this.recentStorylets.push(index);
            this.storylet = this.model.storylets[index];
            if (this.storylet.name) {
                await transaction.setStorylet(this.storylet.name);
            }
        }
    }

    private async evaluateAvailableStorylets(transaction: Transaction): Promise<Choice[]> {
        const choices: Choice[] = [];

        for (let i = 0; i < this.model.storylets.length; i++) {
            const storylet = this.model.storylets[i];
            if (storylet.label && (storylet.condition === undefined || await evaluateLogical(storylet.condition, this.model, transaction))) {
                const label = await evaluateTemplate(storylet.label, this.model, transaction);
                const description = storylet.description && await evaluateTemplate(storylet.description, this.model, transaction);
                const icon = storylet.icon && await evaluateConditional(storylet.icon, this.model, transaction);
                choices.push({
                    id: i,
                    label,
                    description,
                    icon,
                });
            }
        }

        return choices;
    }

    private async stateWithChoices(choices: Choice[], prompt: Template | undefined, transaction: Transaction): Promise<RuntimeSessionState> {
        const locationLabel = this.location && await evaluateTemplate(this.location.label, this.model, transaction);
        const locationDescription = this.location?.description && await evaluateTemplate(this.location.description, this.model, transaction);

        const storyletLabel = this.storylet?.label && await evaluateTemplate(this.storylet.label, this.model, transaction);

        this.staleBody = true;
        this.staleAssignments = true;

        return {
            location: this.location && {
                label: locationLabel!,
                description: locationDescription,
            },
            storylet: this.storylet && {
                label: storyletLabel,
            },
            body: this.body.length ? [...this.body] : undefined,
            assignments: this.assignments.length ? [...this.assignments] : undefined,
            choices,
            prompt: prompt && await evaluateTemplate(prompt, this.model, transaction),
        };
    }

    private async stateWithContinue(label: true | Template, prompt: Template | undefined, transaction: Transaction): Promise<RuntimeSessionState> {
        const locationLabel = this.location && await evaluateTemplate(this.location.label, this.model, transaction);
        const locationDescription = this.location?.description && await evaluateTemplate(this.location.description, this.model, transaction);

        const storyletLabel = this.storylet?.label && await evaluateTemplate(this.storylet.label, this.model, transaction);

        this.staleBody = true;
        this.staleAssignments = true;

        return {
            location: this.location && {
                label: locationLabel!,
                description: locationDescription,
            },
            storylet: this.storylet && {
                label: storyletLabel,
            },
            body: this.body.length ? [...this.body] : undefined,
            assignments: this.assignments.length ? [...this.assignments] : undefined,
            continue: label === true ? true : await evaluateTemplate(label, this.model, transaction),
            prompt: prompt && await evaluateTemplate(prompt, this.model, transaction),
        };
    }

}
