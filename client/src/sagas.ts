import { takeEvery, all, call } from 'redux-saga/effects'
import { login } from './auth';

function* watchLogin() {
    yield takeEvery('Login', login)
}

export default function* rootSaga(): IterableIterator<any> {
    yield all([
        call(watchLogin)
    ]);
}
