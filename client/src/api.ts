import axios from 'axios';
import { UserInfo, HomeResource, Poll, PollProgressEnum} from './types';

const BASE_URL = 'http://localhost:8000/api';

export async function getHomeResource(token: string): Promise<HomeResource> {
  const response = await axios({
    method: 'get',
    url: `${BASE_URL}/private/user-info`,
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  const polls: Poll[] = response.data.polls.map((p: any) => {
    const {id, email, title, createdAt, updatedAt} = p;
    let progress = null;
    if (p.progress === 'in_progress') {
      progress = PollProgressEnum.InProgress;
    } else if (p.progress === 'finished') {
      progress = PollProgressEnum.Finished;
    } else {
      progress = PollProgressEnum.NotStarted;
    }
    return { id, email, title, progress, createdAt, updatedAt };
  });
  const {inviteIds} = response.data;

  return {
    polls,
    inviteIds
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
  const {email, email_verified, name, locale, picture} = response.data;

  const userInfo = {
    email,
    emailVerified: email_verified,
    name,
    locale,
    picture
  };
  return userInfo;
}