import * as React from 'react'
import * as ReactDOM from 'react-dom';
import { App } from './client/App';

console.info('client init');

const root = document.getElementById('root');

if (root === null) {
    console.error('Brakuje root-a');
} else {
    ReactDOM.render(<App />, root);
}