import auth0 from 'auth0-js';

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
    clientID: 'Y8jd6mTa83Z6dVpQQyukniATeI3B4sna',
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
        state: stateString,
        nonce: nonceString
    });
}