import { RouterState } from 'connected-react-router';

export interface UserInfo {
  email: string,
  email_verified: boolean | null,
  name: string | null,
  locale: string | null,
  picture: string | null,
}

enum ProgressEnum { NotStarted, InProgress, Finished }

export interface Poll {
  id: number,
  email: string,
  title: string,
  pollType: string,
  currentProgress: ProgressEnum,
  createdAt: string,
  updatedAt: string,
}

export interface Polls {
  [pollId: number]: Poll | null
}

export interface Proposal {
  summary: string,
  fullDescriptionLink: string | null,
  pollId: number,
  createdAt: string,
  updatedAt: string,
}

export interface Proposals {
  [pollId: number]: Array<Proposal> | null
}

export interface InviteIds {
  // email to poll id
  [email: string]: Array<number>
}

export interface State {
  accessToken: string | null,
  userInfo: UserInfo | null,
  polls: Polls,
  proposals: Proposals,
  inviteIds: InviteIds
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

  export interface Initialize extends BaseAction {
    type: "Initialize",
    accessToken: string | null
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

export type Action = Actions.NoOp | Actions.Login | Actions.AuthCallback | Actions.LogOut | Actions.UserInfoAction | Actions.Initialize;