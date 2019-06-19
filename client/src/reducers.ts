import { combineReducers } from 'redux'
import { connectRouter } from 'connected-react-router'
import { History } from 'history';
import { State, Action } from './types';

const initialState: State = {
  accessToken: null,
  userInfo: null
};

function primaryReducer(state = initialState, action: Action): State {
  if (action.source !== 'internal') {
    return state;
  }

  switch (action.type) {
    case "UserInfo":
      state.userInfo = action.userInfo;
      state.accessToken = action.accessToken;
      return state
    case "LogOut":
      state.accessToken = null;
      state.userInfo = null;
      return state;
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


