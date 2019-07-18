import { push } from 'connected-react-router';
import { select } from 'redux-saga/effects';
import _ from 'lodash';
import { take, put, takeLeading, takeEvery, all, call } from 'redux-saga/effects'
import { login, logOut } from './auth';
import { Polls, UserInfoAction, AuthCallbackAction, InitializeAction, UserInfo, HomeResource, State, HomeResourceResponseAction, RequestHomeResourceAction } from './types';
import { getUserInfo, getHomeResource } from './api';

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
      const userInfoAction : UserInfoAction = { 
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
      const userInfoAction: UserInfoAction = { 
        source: 'internal',
        type: 'UserInfo',
        userInfo: userInfo,
        accessToken: action.accessToken 
      };
      yield put(userInfoAction);
      yield put(push('/app'));
    } else {
      yield put({ source: 'internal', type: 'LogOut' });
    }
  } else {
      yield put({ source: 'internal', type: 'LogOut' });
  }
}

function* requestHomeResource(action: RequestHomeResourceAction) {
  let maybeUserInfo: UserInfo | null = yield select(state => state.primary.userInfo);

  if (!maybeUserInfo) {
    yield take('UserInfo');
  }
  let homeResource: HomeResource | null = null;
  let state: State = yield select(state => state.primary);

  if (state.accessToken != null && state.userInfo != null) {
    try {
      homeResource = yield getHomeResource(state.accessToken);
    } catch (e) {
      console.error("Unable to retrieve user info", e);

      yield put({
        source: 'internal',
        type: 'NoOpResponse',
        uuid: action.uuid,
      });
    }

    if (homeResource != null) {
      const invitePollIds = {
        [state.userInfo.email]: homeResource.invitePollIds
      };
      const polls: Polls = _.keyBy(homeResource.polls, 'id');

      const homeResourceResponse: HomeResourceResponseAction = {
        source: 'internal',
        uuid: action.uuid,
        type: 'HomeResourceResponse',
        invitePollIds,
        polls
      };

      yield put(homeResourceResponse);
    }
  }
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
    call(watchLogin),
    call(watchLogOut),
    call(watchAuthCallback),
    call(watchInitialize),
    call(watchRequestHomeResource)
  ]);
}
