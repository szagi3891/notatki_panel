import * as React from 'react';

interface CreateContextResult<T> {
    AppContext: React.Context<T>,
    useContext: () => T
}

export const createContext = <T>(label: string): CreateContextResult<T> => {
    const marker = {};

    //@ts-expect-error
    const AppContext = React.createContext<T>(marker);

    const useContext = (): T => {
        const context = React.useContext(AppContext);

        if (context === marker) {
            throw Error(`value was read out of context => "${label}"`);
        }

        return context;
    };

    return {
        AppContext,
        useContext
    };
};
