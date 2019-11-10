import { combineReducers } from 'redux'
import _ from 'lodash';
import { connectRouter } from 'connected-react-router'
import { History } from 'history';
import produce from 'immer';
import { State, Action } from './types';

const initialState: State = {
  accessToken: window.localStorage.getItem("token") || null,
  userInfo: null,
  polls: {},
  proposals: {},
  invitePollIds: {},
  createPollRequest: {
    type: 'NotStartedRequestStatus'
  }
};

function primaryReducer(state = initialState, action: Action): State {
  if (action.source !== 'internal') {
    return state;
  }

  return produce(state, draft => {
    switch (action.type) {
      case "UserInfo":
        draft.userInfo = action.userInfo;
        draft.accessToken = action.accessToken;
        return draft;

      case "LogOut":
        draft.accessToken = null;
        draft.userInfo = null;
        return draft;

      case "NoOpResponse":
        return draft;

      case "HomeResourceResponse":
        const inviteIds = _.mergeWith(draft.invitePollIds, action.invitePollIds, (draftIds: number[], actionIds: number[]) => _.uniq((draftIds || []).concat(actionIds)));

        return _.assign(draft, {
          polls: _.assign(draft.polls, action.polls),
          inviteIds
        });
      case "Initialize":
        return draft;

      case "Login":
        return draft;

      case "AuthCallback":
        return draft;

      case "RequestHomeResource":
        return draft;

      case "NoOp":
        return draft;
    }
  });
}

export default (history: History) => combineReducers({
  router: connectRouter(history),
  primary: primaryReducer
})


