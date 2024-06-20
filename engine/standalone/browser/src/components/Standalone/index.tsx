import React from 'react';
import { useSession } from '../../hooks/useSession';
import type { RuntimeSessionState } from '@worldtreeengine/runtime.api';

import { Fullscreen } from '../Fullscreen';
import { SystemMenu } from '../SystemMenu';
import { SplashScreen } from '../SplashScreen';
import { useModel } from '../../hooks/useModel';
import { Game } from '../Game';

interface State {
    counter: number;
    runtimeSessionState?: RuntimeSessionState;
}

function reduce(state: State, runtimeSessionState: RuntimeSessionState | undefined): State {
    return { ...state, runtimeSessionState, counter: state.counter + 1 };
}

export function Standalone({ isNew }: { isNew?: boolean }) {
    const session = useSession();
    const model = useModel();

    const [ state, dispatch ] = React.useReducer(reduce, { counter: 0 });

    const choose = React.useCallback((id: number) => {
        session.choose(id).then(dispatch).catch(console.error);
    }, [ session ]);

    const doContinue = React.useCallback(() => {
        session.continue().then(dispatch).catch(console.error);
    }, [ session ]);

    React.useEffect(() => {
        session.continue().then(dispatch).catch(console.error);
    }, []);

    const [ splashShown, setSplashShown ] = React.useState(true);

    const closeSplash = React.useCallback(() => {
        setSplashShown(false);
    }, [ setSplashShown ]);

    const [ canResume, setCanResume ] = React.useState(!isNew);
    const [ resetCount, setResetCount ] = React.useState(0);

    const newGame = React.useCallback(() => {
        dispatch(undefined);
        closeSplash();
        setResetCount((resetCount) => resetCount + 1);

        setTimeout(() => {
            setCanResume(true);
            session.reset().then(() => session.continue()).then(dispatch).catch(console.error);
        }, 2_000);
    }, [ session, dispatch, closeSplash, setResetCount, setCanResume ]);

    React.useEffect(() => {
        const listener = (event: KeyboardEvent) => {
            if (event.key === 'Escape') {
                setSplashShown((splashShown) => !splashShown || !canResume);
                event.preventDefault();
            }
        }

        window.addEventListener('keydown', listener);
        return () => window.removeEventListener('keydown', listener);
    }, [ setSplashShown, canResume ]);

    return <Fullscreen className="main">
        <SplashScreen show={splashShown} title={model.meta.title || ["Untitled"]} description={model.meta.description} credits={model.meta.credits} onNew={newGame} onResume={canResume ? closeSplash : undefined}/>
        { state.runtimeSessionState && <Game show={!splashShown} state={state.runtimeSessionState} key={resetCount} counter={state.counter} onChoose={choose} onContinue={doContinue}/> }
        <SystemMenu/>
    </Fullscreen>
}
