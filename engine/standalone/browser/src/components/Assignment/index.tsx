import React from 'react';
import { RuntimeSessionState } from '@worldtreeengine/runtime.api';
import { TextBlock } from '../TextBlock';

import './index.css';
import { InlineTextSpan } from '../InlineTextSpan';

export function Assignment({ assignment }: {
    assignment: Exclude<RuntimeSessionState['assignments'], undefined>[number];
}) {
    return <div className="quality-assignment">
        { assignment.results &&
            <ul className="quality-assignment-list">
                { assignment.results.map((changed, i) =>
                        <li className="quality-assignment-list-item" key="i">
                            <div className="quality-assignment-icon">
                                <svg ari-hidden width="16" height="16" viewBox="0 0 16 16" version="1.1" xmlns="http://www.w3.org/2000/svg">
                                    <path d={
                                        changed.operation === 'set' ?
                                            'M8,2c3.311,0 6,2.689 6,6c0,3.311 -2.689,6 -6,6c-3.311,0 -6,-2.689 -6,-6c0,-3.311 2.689,-6 6,-6Zm-1,7l0,3l2,-0l0,-3l3,-0l0,-2l-3,-0l0,-3l-2,-0l0,3l-3,-0l0,2l3,-0Z' :
                                        changed.operation === 'unset' ?
                                            'M8,2c3.311,0 6,2.689 6,6c0,3.311 -2.689,6 -6,6c-3.311,0 -6,-2.689 -6,-6c0,-3.311 2.689,-6 6,-6Zm-1.414,6l-2.122,2.121l1.415,1.415l2.121,-2.122l2.121,2.122l1.415,-1.415l-2.122,-2.121l2.122,-2.121l-1.415,-1.415l-2.121,2.122l-2.121,-2.122l-1.415,1.415l2.122,2.121Z' :
                                        changed.operation === 'increment' ?
                                            'M8,2c3.311,0 6,2.689 6,6c0,3.311 -2.689,6 -6,6c-3.311,0 -6,-2.689 -6,-6c0,-3.311 2.689,-6 6,-6Zm0,2l-4,4l2,-0l0,3l4,-0l0,-3l2,-0l-4,-4Z' :
                                        // changed.operation === 'decrement' ?
                                            'M8,2c3.311,0 6,2.689 6,6c0,3.311 -2.689,6 -6,6c-3.311,0 -6,-2.689 -6,-6c0,-3.311 2.689,-6 6,-6Zm-0,10l4,-4l-2,-0l-0,-3l-4,-0l-0,3l-2,-0l4,4Z'
                                    }/>
                                </svg>
                            </div>
                            <p className="quality-assignment-message" title={JSON.stringify(changed)}>
                                {
                                    changed.value === undefined ? (
                                        changed.style?.possessive ? (
                                            changed.style?.personal ? (
                                                changed.operation === 'set' ?
                                                    <>You now have <InlineTextSpan>{changed.label}</InlineTextSpan></> :
                                                    <>You no longer have <InlineTextSpan>{changed.label}</InlineTextSpan></>
                                            ) : (
                                                changed.style?.plural ? (
                                                    changed.operation === 'set' ?
                                                        <>There are now <InlineTextSpan>{changed.label}</InlineTextSpan></> :
                                                        <>There are no longer <InlineTextSpan>{changed.label}</InlineTextSpan></>
                                                ) : (
                                                    changed.operation === 'set' ?
                                                        <>There is now <InlineTextSpan>{changed.label}</InlineTextSpan></> :
                                                        <>There is no longer <InlineTextSpan>{changed.label}</InlineTextSpan></>
                                                )
                                            )
                                        ) : (
                                            changed.style?.personal ? (
                                                changed.operation === 'set' ?
                                                    <>You are now <InlineTextSpan>{changed.label}</InlineTextSpan></> :
                                                    <>You are no longer <InlineTextSpan>{changed.label}</InlineTextSpan></>
                                            ) : (
                                                changed.operation === 'set' ?
                                                    <>It is now <InlineTextSpan>{changed.label}</InlineTextSpan></> :
                                                    <>It is no longer <InlineTextSpan>{changed.label}</InlineTextSpan></>
                                            )
                                        )
                                    ) : typeof changed.value === 'number' ? (
                                        changed.style?.currency ? (
                                            changed.style?.personal ? (
                                                    changed.value === 0 ?
                                                        <>You no longer have any <InlineTextSpan>{changed.label}</InlineTextSpan></> :
                                                        <>You now have {changed.value} <InlineTextSpan>{changed.label}</InlineTextSpan></>
                                                ) :
                                                changed.value === 1 ?
                                                    <>There is now 1 <InlineTextSpan>{changed.label}</InlineTextSpan></> :
                                                changed.value === 0 ?
                                                    <>There are no longer any <InlineTextSpan>{changed.label}</InlineTextSpan></> :
                                                    <>There are now {changed.value} <InlineTextSpan>{changed.label}</InlineTextSpan></>
                                        ) : changed.style?.personal ? (
                                            changed.style?.plural ?
                                                <>Your <InlineTextSpan>{changed.label}</InlineTextSpan> are now {changed.value}</> :
                                                <>Your <InlineTextSpan>{changed.label}</InlineTextSpan> is now {changed.value}</>
                                        ) : changed.style?.plural ?
                                            <><InlineTextSpan capitalize>{changed.label}</InlineTextSpan> are now {changed.value}</> :
                                            <><InlineTextSpan capitalize>{changed.label}</InlineTextSpan> is now {changed.value}</>
                                    ) : (
                                        changed.style?.personal ? (
                                            changed.style?.plural ? (
                                                changed.operation === 'set' ?
                                                    <>Your <InlineTextSpan>{changed.label}</InlineTextSpan> are now <InlineTextSpan>{changed.value}</InlineTextSpan></> :
                                                    <>Your <InlineTextSpan>{changed.label}</InlineTextSpan> are no longer <InlineTextSpan>{changed.value}</InlineTextSpan></>
                                            ) : (
                                                changed.operation === 'set' ?
                                                    <>Your <InlineTextSpan>{changed.label}</InlineTextSpan> is now <InlineTextSpan>{changed.value}</InlineTextSpan></> :
                                                    <>Your <InlineTextSpan>{changed.label}</InlineTextSpan> is no longer <InlineTextSpan>{changed.value}</InlineTextSpan></>
                                            )
                                        ) : (
                                            changed.style?.plural ? (
                                                changed.operation === 'set' ?
                                                    <><InlineTextSpan capitalize>{changed.label}</InlineTextSpan> are now <InlineTextSpan>{changed.value}</InlineTextSpan></> :
                                                    <><InlineTextSpan capitalize>{changed.label}</InlineTextSpan> are no longer <InlineTextSpan>{changed.value}</InlineTextSpan></>
                                            ) : (
                                                changed.operation === 'set' ?
                                                    <><InlineTextSpan capitalize>{changed.label}</InlineTextSpan> is now <InlineTextSpan>{changed.value}</InlineTextSpan></> :
                                                    <><InlineTextSpan capitalize>{changed.label}</InlineTextSpan> is no longer <InlineTextSpan>{changed.value}</InlineTextSpan></>
                                            )
                                        )
                                    )
                                }.
                            </p>
                        </li>
                    )
                }
            </ul>
        }
        { assignment.description && <div className="quality-assignment-description"><TextBlock>{assignment.description}</TextBlock></div> }
    </div>;
}
