import * as React from 'react'
import * as ReactDOM from 'react-dom';
import { Provider } from './client/Context';
import { App } from './client/App';
import { AppState } from './client/AppState/AppState';

console.info('client init');

const root = document.getElementById('root');

if (root === null) {
    console.error('Brakuje root-a');
} else {
    const appState = new AppState();

    const domJsx = (
        <Provider value={appState}>
            <App />
        </Provider>
    );

    ReactDOM.render(domJsx, root);
}