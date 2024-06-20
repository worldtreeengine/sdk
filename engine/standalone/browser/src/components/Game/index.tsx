import React from 'react';
import { RuntimeSessionState } from '@worldtreeengine/runtime.api';
import { TextBlock } from '../TextBlock';
import { Assignment } from '../Assignment';
import { InlineTextSpan } from '../InlineTextSpan';

import './index.css';
import { Icon } from '../Icon';
import { NonInteractiveTextSpan } from '../NonInteractiveTextSpan';

export function Game({ show, state, counter, onChoose, onContinue }: {
    show: boolean;
    state: RuntimeSessionState;
    counter: number;
    onChoose(id: number): void;
    onContinue(): void;
}) {
    const divRef = React.useRef<HTMLDivElement>(null);
    React.useEffect(() => {
        if (show) {
            divRef.current?.focus();
        }
    }, [ show ]);

    return <div className="game" aria-hidden={!show} inert={show ? undefined : ""} tabIndex={-1} ref={divRef}>
        <div className="game-content">
            <div className="location-title" key={JSON.stringify(state.location?.label || null)}>
                { state.location?.label && <h2><NonInteractiveTextSpan>{ state.location.label }</NonInteractiveTextSpan></h2> }
            </div>

            { state.body &&
                <div className="body" key={JSON.stringify(state.body)}>
                    <TextBlock>{ state.body }</TextBlock>
                </div>
            }

            { state.assignments &&
                <div className="assignments" key={JSON.stringify(state.assignments)}>
                    { state.assignments.map((assignment) => <Assignment assignment={assignment}/>) }
                </div>
            }

            { state.choices &&
                <nav className="choices" key={JSON.stringify(state.choices)}>
                    { state.prompt && <h3 className="choices-prompt"><InlineTextSpan>{ state.prompt }</InlineTextSpan></h3> }
                    <ul>
                        { state.choices.map((choice) =>
                            <li onClick={() => onChoose(choice.id)}>
                                { choice.icon &&
                                    <div className="choice-icon-frame">
                                        <Icon uri={choice.icon}/>
                                    </div>
                                }
                                <div className="choice-content">
                                    <span className="choice-label">
                                        <InlineTextSpan>{choice.label}</InlineTextSpan>
                                    </span>
                                    { choice.description &&
                                        <div className="choice-description"><TextBlock>{choice.description}</TextBlock></div>
                                    }
                                </div>
                            </li>
                        )}
                    </ul>
                </nav>
            }

            { state.continue &&
                <div className="continue">
                    { state.prompt && <h3 className="continue-prompt"><InlineTextSpan>{ state.prompt }</InlineTextSpan></h3> }
                    <button className="continue-button" onClick={onContinue}>{ state.continue === true ? 'Continue' : <InlineTextSpan>{state.continue}</InlineTextSpan> }</button>
                </div>
            }
        </div>
    </div>
}
