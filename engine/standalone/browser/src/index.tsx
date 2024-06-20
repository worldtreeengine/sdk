import './index.css';

import { createRoot } from 'react-dom/client';
import React from 'react';
import { Standalone } from './components/Standalone';
import { Model } from '@worldtreeengine/content.model';
import { LocalRuntimeDriver } from '@worldtreeengine/runtime.drivers.local-driver/src';
import { LocalStorageStateDriver } from '@worldtreeengine/state.drivers.local-storage-driver/src';
import { SessionProvider } from './hooks/useSession';
import { ModelProvider } from './hooks/useModel';

const backgroundColor = document.currentScript?.dataset.backgroundColor;
const foregroundColor = document.currentScript?.dataset.foregroundColor;
const importantForegroundColor = document.currentScript?.dataset.importantForegroundColor;
const highlightBackgroundColor = document.currentScript?.dataset.highlightBackgroundColor;
const highlightForegroundColor = document.currentScript?.dataset.highlightForegroundColor;
const stateKey = document.currentScript?.dataset.stateKey;
const bodyFontFamily = document.currentScript?.dataset.bodyFontFamily;
const labelFontFamily = document.currentScript?.dataset.labelFontFamily;

window.addEventListener('load', () => {
    backgroundColor && document.body.style.setProperty('--background-color', backgroundColor);
    foregroundColor && document.body.style.setProperty('--foreground-color', foregroundColor);
    importantForegroundColor && document.body.style.setProperty('--important-foreground-color', importantForegroundColor);
    highlightBackgroundColor && document.body.style.setProperty('--highlight-background-color', highlightBackgroundColor);
    highlightForegroundColor && document.body.style.setProperty('--highlight-foreground-color', highlightForegroundColor);
    bodyFontFamily && document.body.style.setProperty('--body-font-family', bodyFontFamily);
    labelFontFamily && document.body.style.setProperty('--label-font-family', labelFontFamily);

    const modelScript = document.getElementById('model');

    if (modelScript && modelScript.tagName.toLowerCase() === 'script') {
        const modelJson = modelScript.textContent;

        if (modelJson) {
            const model = JSON.parse(modelJson) as Model;
            const runtime = new LocalRuntimeDriver(model);
            const state = new LocalStorageStateDriver(stateKey || 'state');

            runtime.begin(state).then((session) => {
                const root = document.createElement('div');
                document.body.appendChild(root);
                const reactRoot = createRoot(root);
                reactRoot.render(
                    <ModelProvider model={model}>
                        <SessionProvider session={session}>
                            <Standalone isNew={state.isNew}/>
                        </SessionProvider>
                    </ModelProvider>);
            });
        }
    }
});

/*
@license GPL-3.0-or-later

https://github.com/worldtreeengine/sdk

Copyright © 2024 Doug Valenta.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or (at
your option) any later version.

This program is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program. If not, see <https://www.gnu.org/licenses/>.
*/
