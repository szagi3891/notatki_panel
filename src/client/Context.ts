import { AppState } from "./AppState/AppState";
import { createContext } from "./utils/createContext";

const {AppContext, useContext} = createContext<AppState>('appState');

export const Provider = AppContext.Provider;

export const useAppStateContext = (): AppState => {
    return useContext();
};
