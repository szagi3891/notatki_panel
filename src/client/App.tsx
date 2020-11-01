import * as React from 'react';
import { observer } from 'mobx-react';
import { useAppStateContext } from './Context';

export const App = observer(() => {
    const appState = useAppStateContext();
    const [ state, setState ] = React.useState(false);

    // React.useEffect(() => {
    //     const timer = setInterval(() => {
    //         appState.inc();
    //     }, 1000);

    //     return () => {
    //         clearInterval(timer);
    //     };
    // });

    return (
        <div>
            <div onClick={() => setState(!state)}>toogle {state}</div>
            { state ? <div>to jest applikacja {appState.counter.counter}</div> : null }
        </div>
    );
});

