import { RouterState } from 'connected-react-router';

export interface UserInfo {
  email: string,
  email_verified: boolean | null,
  name: string | null,
  locale: string | null,
  picture: string | null,
}

export interface State {
  accessToken: string | null,
  userInfo: UserInfo | null
}

export namespace Actions {
  interface BaseAction {
    source: "internal"
  }

  export interface AuthCallback extends BaseAction {
    type: "AuthCallback",
    accessToken: string,
    state: string
  }

  export interface Login extends BaseAction {
    type: "Login"
  }

  export interface UserInfoAction extends BaseAction {
    type: "UserInfo",
    accessToken: string,
    userInfo: UserInfo
  }

  export interface LogOut extends BaseAction {
    type: "LogOut"
  }

  export interface NoOp extends BaseAction {
    type: "NoOp"
  }
}

export interface CombinedState {
  router: RouterState,
  primary: State
}

export type Action = Actions.NoOp | Actions.Login | Actions.AuthCallback | Actions.LogOut | Actions.UserInfoAction;