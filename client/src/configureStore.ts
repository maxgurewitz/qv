import { createBrowserHistory } from 'history'
import { applyMiddleware, compose, createStore } from 'redux'
import { routerMiddleware } from 'connected-react-router'
import createSagaMiddleware from 'redux-saga'
import { DeepPartial } from 'redux';
import createRootReducer from './reducers'
import { CombinedState } from './types';
import rootSaga from './sagas';

export const history = createBrowserHistory();

export default function configureStore(preloadedState?: DeepPartial<CombinedState>) {
    const composeEnhancer: typeof compose = (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || compose;
    const sagaMiddleware = createSagaMiddleware();

    const store = createStore(
        createRootReducer(history),
        preloadedState,
        composeEnhancer(
            applyMiddleware(
                routerMiddleware(history),
                sagaMiddleware
            ),
        ),
    );
    sagaMiddleware.run(rootSaga);

    return store;
}
