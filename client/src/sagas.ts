import { put, takeLeading, takeEvery, all, call } from 'redux-saga/effects'
import { login, logOut } from './auth';
import { Actions } from './types';
import { getUserInfo } from './api';
import { push } from 'connected-react-router';

function* watchLogin() {
  yield takeEvery('Login', login)
}

function* onAuthCallback(action: Actions.AuthCallback) {
  const storedAuthState = window.localStorage.getItem('state');

  let userInfoResponse = null;
  try {
    userInfoResponse = yield getUserInfo(action.accessToken);
  } catch (e) {
    console.error(e);
  }
  if (userInfoResponse == null) {
    yield put({ source: 'internal', type: 'LogOut' });
  } else {
    const userInfo = userInfoResponse.data;

    if (action.state !== storedAuthState) {
      yield put({ source: 'internal', type: 'LogOut' });
    } else {
      window.localStorage.setItem("token", action.accessToken);
      yield put({ source: 'internal', type: 'UserInfo', userInfo, accessToken: action.accessToken });
      yield put(push('/app'));
    }
  }
}

function* watchAuthCallback() {
  yield takeLeading('AuthCallback', onAuthCallback);
}

function* watchLogOut() {
  yield takeLeading('LogOut', logOut);
}

export default function* rootSaga(): IterableIterator<any> {
  yield all([
    call(watchLogin),
    call(watchLogOut),
    call(watchAuthCallback)
  ]);
}
