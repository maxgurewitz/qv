import { combineReducers } from 'redux'
import _ from 'lodash';
import { connectRouter } from 'connected-react-router'
import { History } from 'history';
import produce from 'immer';
import { State, Action } from './types';
import { AxiosError } from 'axios';
import { object } from 'prop-types';

const initialState: State = {
  accessToken: window.localStorage.getItem("token") || null,
  userInfo: null,
  polls: {},
  proposals: {},
  invitePollIds: {},
  initializeRequest: {
    type: 'NotStartedRequestStatus'
  },
  createPollRequest: {
    type: 'NotStartedRequestStatus'
  }
};

function isAxiosError(val: any): val is AxiosError<any> {
  return !!val.isAxiosError;
}

function primaryReducer(state = initialState, action: Action): State {
  if (action.source !== 'internal') {
    return state;
  }

  return produce(state, draft => {
    switch (action.type) {
      case 'UserInfo':
        draft.userInfo = action.userInfo;
        draft.accessToken = action.accessToken;
        return draft;

      case 'LogOut':
        draft.accessToken = null;
        draft.userInfo = null;
        return draft;

      case 'HomeResourceResponse':
        const inviteIds = _.mergeWith(draft.invitePollIds, action.invitePollIds, (draftIds: number[], actionIds: number[]) => _.uniq((draftIds || []).concat(actionIds)));

        return _.assign(draft, {
          polls: _.assign(draft.polls, action.polls),
          inviteIds
        });

      case 'CreatePoll':
        draft.createPollRequest = {
          type: 'InProgressRequestStatus',
        };
        return draft;

      case 'CreatePollResponse':
        if (isAxiosError(action.response)) {
          draft.createPollRequest = {
            type: 'FailedRequestStatus',
            error: action.response
          };
        } else {
          draft.createPollRequest = {
            type: 'SuccessfulRequestStatus',
            response: action.response
          };

          draft.polls[action.response.id] = action.response;
        }
        return draft;

      default:
        return draft;
    }
  });
}

export default (history: History) => combineReducers({
  router: connectRouter(history),
  primary: primaryReducer
})


