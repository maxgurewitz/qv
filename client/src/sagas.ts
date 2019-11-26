import { push } from 'connected-react-router';
import { select } from 'redux-saga/effects';
import _ from 'lodash';
import { race, delay, take, put, takeLeading, takeEvery, all, call } from 'redux-saga/effects'
import { login, logOut } from './auth';
import { Polls, UserInfoAction, AuthCallbackAction, InitializeAction, UserInfo, HomeResource, State, HomeResourceResponseAction, RequestHomeResourceAction, CombinedState, CreatePollAction, Poll, CreatePollResponseAction } from './types';
import { getUserInfo, getHomeResource, createPoll } from './api';
import { AxiosError } from 'axios';

function* watchLogin() {
  yield takeEvery('Login', login)
}

function* onAuthCallback(action: AuthCallbackAction) {
  const storedAuthState = window.localStorage.getItem('state');

  let userInfo = null;
  try {
    userInfo = yield getUserInfo(action.accessToken);
  } catch (e) {
    console.error("Unable to retrieve user info", e);
  }
  if (userInfo == null) {
    yield put({ source: 'internal', type: 'LogOut' });
  } else {
    if (action.state !== storedAuthState) {
      yield put({ source: 'internal', type: 'LogOut' });
    } else {
      window.localStorage.setItem("token", action.accessToken);
      const userInfoAction: UserInfoAction = {
        source: 'internal',
        type: 'UserInfo',
        userInfo,
        accessToken: action.accessToken
      };
      yield put(userInfoAction);
      yield put(push('/app'));
    }
  }
}

function* initialize(action: InitializeAction) {
  if (action.accessToken != null) {
    let userInfo: UserInfo | null = null;
    try {
      userInfo = yield getUserInfo(action.accessToken);
    } catch (e) {
      console.error("Unable to retrieve user info", e);
    }

    if (userInfo !== null) {
      let path: string = yield select((state: CombinedState) => {
        return state.router.location.pathname;
      });
      const userInfoAction: UserInfoAction = {
        source: 'internal',
        type: 'UserInfo',
        userInfo: userInfo,
        accessToken: action.accessToken
      };
      yield put(userInfoAction);
      yield put(push(path));
    } else {
      yield put({ source: 'internal', type: 'LogOut' });
    }
  } else {
    yield put({ source: 'internal', type: 'LogOut' });
  }
}

function* handleApiError(e: AxiosError) {
  console.error('Api request failed', e);

  if (e.code === '401' || e.code === '403') {
    yield put({ type: 'LogOut' });
  }
}

function* requestHomeResource(action: RequestHomeResourceAction) {
  let state: State = yield select(state => state.primary);

  if (state.accessToken != null && state.userInfo != null) {
    let homeResource: HomeResource | null = null;
    try {
      homeResource = yield getHomeResource(state.accessToken);
    } catch (e) {
      yield handleApiError(e);
    }

    if (homeResource != null) {
      const invitePollIds = {
        [state.userInfo.email]: homeResource.invitePollIds
      };
      const polls: Polls = _.keyBy(homeResource.polls, 'id');

      const homeResourceResponse: HomeResourceResponseAction = {
        source: 'internal',
        type: 'HomeResourceResponse',
        invitePollIds,
        polls
      };

      yield put(homeResourceResponse);
    }
  }
}

function* onCreatePollCallback(action: CreatePollAction) {
  let state: State = yield select(state => state.primary);
  let response: Poll | AxiosError;

  if (state.accessToken != null && state.userInfo != null) {
    let poll: Poll | null = null;

    const pollPayload = {
      title: action.title,
      pollType: 'qv',
      summary: action.summary,
      fullDescriptionLink: action.fullDescriptionLink,
    };

    try {
      poll = yield createPoll(state.accessToken, pollPayload);
      response = poll as Poll;
    } catch (e) {
      yield handleApiError(e);
      response = e;
    }

    const createPollResponse: CreatePollResponseAction = {
      source: 'internal',
      type: 'CreatePollResponse',
      response
    };

    yield put(createPollResponse);

    if (poll !== null) {
      yield put(push(`/update-poll/${poll.id}`));
    }
  }
}

function* watchCreatePoll() {
  yield takeLeading('CreatePoll', onCreatePollCallback);
}

function* watchAuthCallback() {
  yield takeLeading('AuthCallback', onAuthCallback);
}

function* watchInitialize() {
  yield takeLeading('Initialize', initialize);
}

function* watchLogOut() {
  yield takeLeading('LogOut', logOut);
}

function* watchRequestHomeResource() {
  yield takeLeading('RequestHomeResource', requestHomeResource);
}

export default function* rootSaga(): IterableIterator<any> {
  yield all([
    call(watchCreatePoll),
    call(watchLogin),
    call(watchLogOut),
    call(watchAuthCallback),
    call(watchInitialize),
    call(watchRequestHomeResource)
  ]);
}
