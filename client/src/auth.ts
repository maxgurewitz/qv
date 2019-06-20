import auth0 from 'auth0-js';

const clientID = 'Y8jd6mTa83Z6dVpQQyukniATeI3B4sna';

function generateRandomString(length: number): string {
  const charset = '0123456789ABCDEFGHIJKLMNOPQRSTUVXYZabcdefghijklmnopqrstuvwxyz-._~';
  let result = ''

  while (length > 0) {
    const bytes = new Uint8Array(16);
    const random = window.crypto.getRandomValues(bytes);

    for (let i = 0; i < random.length; i++) {
      const c = random[i];

      if (length === 0) {
        break;
      }
      if (c < charset.length) {
        result += charset[c];
        length--;
      }
    }
  }
  return result;
}

const webAuth = new auth0.WebAuth({
  domain: 'maxthegeek1.auth0.com',
  clientID,
  redirectUri: 'http://localhost:3000/auth-callback',
  responseType: 'token',
  scope: 'openid email profile'
});

export function login() {
  const nonceString = generateRandomString(16);
  window.localStorage.setItem('nonce', nonceString);
  
  const stateString = generateRandomString(16);
  window.localStorage.setItem('state', stateString);

  webAuth.authorize({
    nonce: nonceString,
    state: stateString
  });
}

// FIXME I found the solution, the required configuration in my case was in the tenant section. I had forgotten to test allowed logout url with the full logout route such has http://*.MY_APP_DOMAIN.com/logout
// https://community.auth0.com/t/log-out-returnto-parameter-shows-oops-something-went-wrong-page/17758
// https://auth0.com/blog/beyond-create-react-app-react-router-redux-saga-and-more/#Securing-Your-React-Application
export function logOut() {
  window.localStorage.removeItem('nonce');
  window.localStorage.removeItem('state');
  window.localStorage.removeItem('token');

  webAuth.logout({
    returnTo: 'http://localhost:3000',
    clientID
  });
}