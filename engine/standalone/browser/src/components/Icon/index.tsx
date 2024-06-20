import React from 'react';

import './index.css';

export function Icon({ uri }: { uri: string}) {
    const url = React.useMemo(() => new URL(uri, document.location.toString()), [ uri ]);

    if (url.protocol === 'game-icons:') {
        const maskIconUrl = `url(https://cdn.jsdelivr.net/gh/game-icons/icons@master/${url.pathname}.svg)`;
        let maskIconColor = url.hash || undefined;
        if (maskIconColor && !maskIconColor.match(/^#[0-9a-f]{3}$|^#[0-9a-f]{6}$/)) {
            maskIconColor = maskIconColor.substring(1);
        }
        const maskIconStyle = {
            "--mask-icon-url": maskIconUrl,
            "--mask-icon-color": maskIconColor,
        } as React.CSSProperties;

        return <div className="icon mask-icon" style={maskIconStyle}/>;
    } else {
        return <img className="icon img-icon" src={uri}/>
    }
}
