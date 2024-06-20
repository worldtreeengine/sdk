import React from 'react';
import { RuntimeSession } from '@worldtreeengine/runtime.api';

const SessionContext = React.createContext<RuntimeSession | null>(null);

export function useSession() {
    const session = React.useContext(SessionContext);
    if (!session) {
        throw new Error('useSession called outside a SessionProvider');
    }
    return session;
}

export function SessionProvider ({ session, children }: {
    session: RuntimeSession,
    children: React.ReactNode,
}) {
    return <SessionContext.Provider value={session}>{children}</SessionContext.Provider>;
}
