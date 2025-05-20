import { graphql, setSession } from '$houdini';
import { redirect, type Handle } from '@sveltejs/kit';

const PRIVATE_ROUTES = [
  '/me',
  '/favorites',
  '/create',
  '/stats/',
  /^\/chats\/.*$/
];

export const handle: Handle = async ({ event, resolve }) => {
  const { pathname } = event.url;
  let accessToken    = event.cookies.get('accessToken');
  let expiresAt      = event.cookies.get('expiresAt');
  const refreshToken = event.cookies.get('refreshToken');

  console.log('Incoming request:', { path: pathname, accessToken, expiresAt, refreshToken });
  setSession(event, { accessToken });
  const tokenExpired = expiresAt && parseInt(expiresAt, 10) < Date.now();
  const needsRefresh = !accessToken || tokenExpired;

  if (needsRefresh && refreshToken) {
    console.log('Attempting refresh…');
    event.cookies.delete('accessToken', { path: '/' });
    event.cookies.delete('expiresAt',   { path: '/' });

    try {
      const refresh = graphql(`
        mutation refresh($refreshToken: String!) {
          refresh(refreshToken: $refreshToken) {
            accessToken
            refreshToken
          }
        }
      `);
      const res = await refresh.mutate({ refreshToken }, { event });
      console.log('Refresh response:', res);

      if (res.errors || !res.data?.refresh) {
        throw new Error('Refresh failed');
      }

      accessToken       = res.data.refresh.accessToken;
      const newRefresh  = res.data.refresh.refreshToken;
      const newExpires  = Date.now() + 100 * 60 * 1000; // e.g. 100 min

      event.cookies.set('accessToken', accessToken, {
        path: '/', httpOnly: true, sameSite: 'strict', maxAge: 60 * 100
      });
      event.cookies.set('refreshToken', newRefresh, {
        path: '/', httpOnly: true, sameSite: 'strict', maxAge: 60 * 180
      });
      event.cookies.set('expiresAt', newExpires.toString(), { path: '/' });

      setSession(event, { accessToken });
    } catch (err) {
      console.error('Refresh error:', err);
      event.cookies.delete('refreshToken', { path: '/' });
      event.cookies.delete('userId',       { path: '/' });
      accessToken = undefined;
    }
  }

  const isPrivate = PRIVATE_ROUTES.some(route =>
    typeof route === 'string'
      ? pathname === route
      : route.test(pathname)
  );

  if (isPrivate && !accessToken) {
    throw redirect(302, '/login');
  }

  return resolve(event);
};
