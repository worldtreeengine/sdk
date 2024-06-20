import React from 'react';
import { useFullscreen } from '../Fullscreen';
import { useSession } from '../../hooks/useSession';
import './index.css';

export function SystemMenu() {
    const { isFullscreenAvailable, isFullscreen, toggleFullscreen } = useFullscreen();

    const fullscreenLabel = isFullscreen ? 'Exit fullscreen' : 'Enter fullscreen';

    return <nav className="system-menu">
        <ul>
            { isFullscreenAvailable &&
                <li>
                    <button className="system-menu-button system-menu-button-fullscreen" onClick={toggleFullscreen} aria-label={fullscreenLabel} title={fullscreenLabel}>
                        <svg aria-hidden width="24" height="24" viewBox="0 0 24 24" version="1.1" xmlns="http://www.w3.org/2000/svg">
                            { isFullscreen ?
                                <path d="M8,9l-6,0l-0,-2l4,0l-0,-3l2,0l0,5Zm8,0l0,-5l2,-0l0,3l4,-0l0,2l-6,0Zm-8,6l0,5l-2,0l0,-3l-4,0l-0,-2l6,-0Zm8,0l6,0l0,2l-4,0l0,3l-2,0l0,-5Z"/>
                                :
                                <path d="M2,4l6,0l0,2l-4,0l0,3l-2,0l0,-5Zm20,0l0,5l-2,0l0,-3l-4,0l0,-2l6,0Zm-20,16l0,-5l2,0l0,3l4,0l0,2l-6,0Zm20,0l-6,0l0,-2l4,0l0,-3l2,0l0,5Z"/>
                            }
                        </svg>
                    </button>
                </li>
            }
        </ul>
    </nav>
}
