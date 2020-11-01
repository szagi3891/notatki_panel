import * as React from 'react';
import { observer } from 'mobx-react';
import { useAppStateContext } from './Context';

export const App = observer(() => {
    const appState = useAppStateContext();
    const [ state, setState ] = React.useState(false);

    return (
        <div>
            <div onClick={() => setState(!state)}>toogle {state}</div>
            { state ? <div>to jest applikacja {appState.counterValue}</div> : null }
        </div>
    );
});

