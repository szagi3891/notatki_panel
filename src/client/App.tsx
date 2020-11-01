import * as React from 'react';
import { observer } from 'mobx-react';
import { useAppStateContext } from './Context';

export const App = observer(() => {
    const appState = useAppStateContext();

    React.useEffect(() => {
        const timer = setInterval(() => {
            appState.inc();
        }, 1000);

        return () => {
            clearInterval(timer);
        };
    });

    return (
        <div>
            to jest applikacja {appState.counter}
        </div>
    );
});

