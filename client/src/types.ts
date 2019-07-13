import { RouterState } from 'connected-react-router';

export interface HomeResource {
  polls: Poll[],
  inviteIds: number[]
}

export interface UserInfo {
  email: string,
  emailVerified: boolean | null,
  name: string | null,
  locale: string | null,
  picture: string | null,
}

export enum PollProgressEnum { NotStarted, InProgress, Finished }

export interface Poll {
  id: number,
  email: string,
  title: string,
  pollType: string,
  currentProgress: PollProgressEnum,
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

interface BaseAction {
  source: "internal"
}

export interface AuthCallbackAction extends BaseAction {
  type: "AuthCallback",
  accessToken: string,
  state: string
}

export interface RequestHomeResourceAction extends BaseAction {
  type: "RequestHomeResource",
}

export interface HomeResourceResponseAction extends BaseAction {
  type: "HomeResourceResponse",
  polls: Polls,
  inviteIds: InviteIds
}

export interface LoginAction extends BaseAction {
  type: "Login"
}

export interface UserInfoAction extends BaseAction {
  type: "UserInfo",
  accessToken: string,
  userInfo: UserInfo
}

export interface InitializeAction extends BaseAction {
  type: "Initialize",
  accessToken: string | null
}

export interface LogOutAction extends BaseAction {
  type: "LogOut"
}

export interface NoOpAction extends BaseAction {
  type: "NoOp"
}

export interface CombinedState {
  router: RouterState,
  primary: State
}

export type Action = 
  NoOpAction |
  InitializeAction |
  LoginAction |
  RequestHomeResourceAction |
  HomeResourceResponseAction |
  LogOutAction | 
  AuthCallbackAction | 
  UserInfoAction
  ;