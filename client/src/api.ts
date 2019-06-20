import axios, { AxiosResponse } from 'axios';
import { UserInfo } from './types';

export function getUserInfo(token: string): Promise<AxiosResponse<UserInfo>> {
  return axios({
    method: 'get',
    url: 'http://localhost:8000/api/private/user-info',
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
}