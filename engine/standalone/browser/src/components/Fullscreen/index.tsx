import React from 'react';

export interface FullscreenState {
    enterFullscreen(): void;
    exitFullscreen(): void;
    toggleFullscreen(): void;
    isFullscreen: boolean;
    isFullscreenAvailable: boolean;
}

const INITIAL_STATE: FullscreenState = {
    enterFullscreen() {},
    exitFullscreen() {},
    toggleFullscreen() {},
    isFullscreen: !!document.fullscreenElement,
    isFullscreenAvailable: document.fullscreenEnabled,
};

const FullscreenContext = React.createContext<FullscreenState>(INITIAL_STATE);

export function Fullscreen(props: React.HTMLProps<HTMLDivElement>) {
    const { children, ...rest } = props;
    const ref = React.useRef<HTMLDivElement>(null);
    const [ isFullscreen, setIsFullscreen ] = React.useState(!!document.fullscreenElement);

    const enterFullscreen = React.useCallback(() => {
        ref.current?.requestFullscreen({
            navigationUI: 'hide',
        }).finally(() => setIsFullscreen(!!document.fullscreenElement));
    }, [ ref, setIsFullscreen ]);

    const exitFullscreen = React.useCallback(() => {
        document.exitFullscreen().finally(() => setIsFullscreen(!!document.fullscreenElement));
    }, [ setIsFullscreen ]);

    const toggleFullscreen = React.useCallback(() => {
        if (isFullscreen) {
            exitFullscreen();
        } else {
            enterFullscreen();
        }
    }, [ isFullscreen, exitFullscreen, enterFullscreen ]);

    const isFullscreenAvailable = document.fullscreenEnabled;

    React.useEffect(() => {
        if (ref.current) {
            const element = ref.current;

            const listener = () => {
                setIsFullscreen(!!document.fullscreenElement)
            }

            element.addEventListener('fullscreenchange', listener);
            return () => element.removeEventListener('fullscreenchange', listener);
        }
    }, [ setIsFullscreen, ref.current ]);

    return <div { ...rest } ref={ ref }>
        <FullscreenContext.Provider value={ { isFullscreen, enterFullscreen, exitFullscreen, toggleFullscreen, isFullscreenAvailable } }>
            { children }
        </FullscreenContext.Provider>
    </div>;
}

export function useFullscreen() {
    return React.useContext(FullscreenContext);
}
