import { push } from 'connected-react-router';
import { select } from 'redux-saga/effects';
import _ from 'lodash';
import { race, delay, take, put, takeLeading, takeEvery, all, call } from 'redux-saga/effects'
import { login, logOut } from './auth';
import { Polls, UserInfoAction, AuthCallbackAction, InitializeAction, UserInfo, HomeResource, State, HomeResourceResponseAction, RequestHomeResourceAction, CombinedState, CreatePollAction } from './types';
// TODO centralize error handling
import { getUserInfo, getHomeResource } from './api';
import { AxiosError } from 'axios';

function* watchLogin() {
  yield takeEvery('Login', login)
}

function* waitForUserInfoPresent() {
  let maybeUserInfo: UserInfo | null = yield select(state => state.primary.userInfo);

  if (!maybeUserInfo) {
    const { userInfoAction } = yield race({
      userInfoAction: take('UserInfo'),
      delay: delay(5000)
    });

    if (!userInfoAction) {
      yield put({ source: 'internal', type: 'LogOut' });
    }
  }
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

  throw e;
}

function* requestHomeResource(action: RequestHomeResourceAction) {
  yield waitForUserInfoPresent();

  let homeResource: HomeResource | null = null;
  let state: State = yield select(state => state.primary);

  if (state.accessToken != null && state.userInfo != null) {
    try {
      homeResource = yield (getHomeResource(state.accessToken).catch(handleApiError));
    } catch (e) { }

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

function onCreatePollCallback(action: CreatePollAction) {

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
