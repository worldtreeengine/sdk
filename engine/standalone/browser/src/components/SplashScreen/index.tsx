import React from 'react';
import './index.css';
import { InlineTextSpan } from '../InlineTextSpan';
import { TextBlock } from '../TextBlock';
import { Text } from '@worldtreeengine/content.model';
import { NonInteractiveTextSpan } from '../NonInteractiveTextSpan';

export function SplashScreen({ show, title, description, credits, onNew, onResume }: {
    show: boolean;
    title: Text;
    description?: Text;
    credits?: Text[];
    onNew(): void;
    onResume?(): void;
}) {
    const creditsRef = React.useRef<HTMLDivElement>(null);

    const divRef = React.useRef<HTMLDivElement>(null);

    React.useEffect(() => {
        if (show) {
            divRef.current?.focus();
        }
    }, [ show ]);

    const generator = React.useMemo(() => {
        return (document.querySelector('meta[name="generator"]') as HTMLMetaElement | null)?.content;
    }, []);

    return <div className="splash" aria-hidden={!show} inert={!show ? "" : undefined} tabIndex={-1} ref={divRef}>
        <div className="splash-content">
            <h1><NonInteractiveTextSpan>{title}</NonInteractiveTextSpan></h1>
            <div className="splash-credits" ref={creditsRef}>
                { credits && credits.map((credit, i) =>
                    <p key={i}>
                        <InlineTextSpan>{credit}</InlineTextSpan>
                    </p>
                ) }
                <p className="splash-credit-engine-credit">Made with <a href="https://github.com/worldtreeengine" target="_blank">Worldtree</a></p>
            </div>
            <nav className="splash-navigation">
                { onResume && <button className="splash-navigation-button splash-navigation-button-resume-game" onClick={onResume}>Continue Game</button> }
                <button className="splash-navigation-button splash-navigation-button-new-game" onClick={onNew}>New Game</button>
            </nav>
            { generator &&
                <p className="splash-version">
                    <a href="https://github.com/worldtreeengine" target="_blank">{ generator }</a>
                </p>
            }
        </div>
    </div>;
}
