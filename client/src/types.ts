import { RouterState } from 'connected-react-router';

export interface State {
    accessToken: string | null
}

export namespace Actions {
    interface BaseAction {
        source: "internal"
    }

    export interface Login extends BaseAction {
        type: "Login"
    }

    export interface NoOp extends BaseAction {
        type: "NoOp"
    }
}

export interface CombinedState {
    router: RouterState,
    primary: State
}

export type Action = Actions.NoOp | Actions.Login;