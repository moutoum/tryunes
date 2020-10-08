import React, {createContext, Dispatch, useReducer} from "react";
import {initialState, reducer, State} from "../reducers/reducer";
import {Action} from "../reducers/definitions";

export const APIContext = createContext<[State, Dispatch<Action>]>([initialState, () => null]);

export const APIProvider: React.FC = ({ children }) => {
    const [state, dispatch] = useReducer(reducer, initialState)
    return (
        <APIContext.Provider value={[state, dispatch]}>
            {children}
        </APIContext.Provider>
    );
}