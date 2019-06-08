import { takeLeading, takeEvery, all, call } from 'redux-saga/effects'
import { login } from './auth';

function* watchLogin() {
    yield takeEvery('Login', login)
}

function onAuthCallback() {

}

function* watchAuthCallback() {
    yield takeLeading('AuthCallback', onAuthCallback);
}

export default function* rootSaga(): IterableIterator<any> {
    yield all([
        call(watchLogin)
    ]);
}
