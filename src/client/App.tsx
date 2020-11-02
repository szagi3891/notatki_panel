import * as React from 'react';
import { observer } from 'mobx-react';
import { useAppStateContext } from './Context';
import styled from '@emotion/styled';

const PathWrapper = styled('div')`
    border: 1px solid red;
    padding: 10px;
    margin-top: 10px;
`;

const Path = observer(() => {
    const appState = useAppStateContext();
    const chunks = appState.currentPath.pathChunks;

    if (chunks.length === 0) {
        return (
            <PathWrapper>--root--</PathWrapper>
        );
    }

    return (
        <PathWrapper>
            { chunks.map((item) => {
                return <div>{ item }</div>
            })}
        </PathWrapper>
    )
});

const ListWrapper = styled('div')`
    border: 1px solid blue;
    padding: 10px;
    margin-top: 10px;
`;

const List = observer(() => {
    const appState = useAppStateContext();
    const listDir = appState.listDir;

    if (listDir === null) {
        return (
            <div>Loading ...</div>
        );
    }


    return (
        <ListWrapper>{
            listDir.map((item) => (
                <div>{item}</div>
            ))
        }</ListWrapper>
    );
});

export const App = observer(() => {
    const appState = useAppStateContext();
    const [ state, setState ] = React.useState(false);

    return (
        <div>
            <div onClick={() => setState(!state)}>toogle {state}</div>
            { state ? <div>to jest applikacja {appState.counterValue}</div> : null }
            <Path />
            <List />
        </div>
    );
});

