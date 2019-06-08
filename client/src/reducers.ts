import redux, { combineReducers } from 'redux'
import { connectRouter } from 'connected-react-router'
import { History } from 'history';
import { State, Action } from './types';

const initialState: State = {
    accessToken: null
};

function primaryReducer(state = initialState, action: Action): State {
    if (action.source !== 'internal') {
        return state;
    }

    switch (action.type) {
        case "Login":
            return state;
        case "AuthCallback":
            return state;
        case "NoOp":
            return state;
    }
}

export default (history: History) => combineReducers({
    router: connectRouter(history),
    primary: primaryReducer
})


