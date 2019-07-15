import { combineReducers } from 'redux'
import _ from 'lodash';
import { connectRouter } from 'connected-react-router'
import { History } from 'history';
import { State, Action } from './types';

const initialState: State = {
  accessToken: window.localStorage.getItem("token") || null,
  userInfo: null,
  polls: {},
  proposals: {},
  inviteIds: {},
  requestsInFlight: new Set()
};

function primaryReducer(state = initialState, action: Action): State {
  if (action.source !== 'internal') {
    return state;
  }

  switch (action.type) {
    case "UserInfo":
      state.userInfo = action.userInfo;
      state.accessToken = action.accessToken;
      return state;

    case "LogOut":
      state.accessToken = null;
      state.userInfo = null;
      return state;

    case "HomeResourceResponse": 
      const inviteIds = _.mergeWith(state.inviteIds, action.inviteIds, (stateIds: number[], actionIds: number[]) =>   
        _.uniq(stateIds.concat(actionIds))
      );

      state.requestsInFlight.delete(action.uuid);

      return _.assign(state, {
        polls: _.assign(state.polls, action.polls),
        inviteIds
      });
    case "Initialize":
      return state;

    case "Login":
      return state;

    case "AuthCallback":
      return state;

    case "RequestHomeResource":
      state.requestsInFlight.add(action.uuid);
      return state;

    case "NoOp":
      return state;
  }
}

export default (history: History) => combineReducers({
  router: connectRouter(history),
  primary: primaryReducer
})


