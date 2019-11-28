import axios from 'axios';
import { UserInfo, HomeResource, Poll, PollProgressEnum, CreatePollAction, CreatePollPayload } from './types';

// TODO replace as part of build
const BASE_URL = 'http://localhost:8000/api';

export async function getHomeResource(token: string): Promise<HomeResource> {
  const response = await axios({
    method: 'get',
    url: `${BASE_URL}/private/home`,
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });

  const polls: Poll[] = response.data.polls.map((p: any) => {
    const { id, email, title, summary, full_description_link, created_at, updated_at } = p;
    let progress = null;
    if (p.progress === 'in_progress') {
      progress = PollProgressEnum.InProgress;
    } else if (p.progress === 'finished') {
      progress = PollProgressEnum.Finished;
    } else {
      progress = PollProgressEnum.NotStarted;
    }
    return {
      id,
      email,
      title,
      progress,
      summary,
      fullDescriptionLink: full_description_link,
      createdAt: created_at,
      updatedAt: updated_at
    };
  });
  const { invite_poll_ids } = response.data;

  return {
    polls,
    invitePollIds: invite_poll_ids
  };
}

export async function getPoll(token: string, requestedId: number): Promise<Poll> {
  const response = await axios({
    method: 'get',
    url: `${BASE_URL}/private/polls/${requestedId}`,
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });

  const { id, email, title, poll_type, summary, full_description_link, current_progress, created_at, updated_at } = response.data.poll;

  return {
    id,
    email,
    title,
    pollType: poll_type,
    summary,
    fullDescriptionLink: full_description_link,
    currentProgress: current_progress,
    createdAt: created_at,
    updatedAt: updated_at
  };
}

export async function createPoll(token: string, payload: CreatePollPayload): Promise<Poll> {
  const body = {
    title: payload.title,
    poll_type: payload.pollType,
    summary: payload.summary,
    full_description_link: payload.fullDescriptionLink,
  };

  const response = await axios({
    method: 'post',
    data: body,
    url: `${BASE_URL}/private/polls`,
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });

  const { id, email, title, poll_type, summary, full_description_link, current_progress, created_at, updated_at } = response.data.poll;

  return {
    id,
    email,
    title,
    pollType: poll_type,
    summary,
    fullDescriptionLink: full_description_link,
    currentProgress: current_progress,
    createdAt: created_at,
    updatedAt: updated_at
  };
}

export async function getUserInfo(token: string): Promise<UserInfo> {
  const response = await axios({
    method: 'get',
    url: `${BASE_URL}/private/user-info`,
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  const { email, email_verified, name, locale, picture } = response.data.user;

  const userInfo = {
    email,
    emailVerified: email_verified,
    name,
    locale,
    picture
  };
  return userInfo;
}