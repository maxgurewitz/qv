import axios, { AxiosResponse } from 'axios';
import { UserInfo } from './types';

export function getUserInfo(token: string): Promise<AxiosResponse<UserInfo>> {
  return axios({
    method: 'post',
    url: 'http://localhost:8000/api/private/user_info',
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
}