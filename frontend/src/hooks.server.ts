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

  let accessToken   = event.cookies.get('accessToken');
  const expiresAt   = event.cookies.get('expiresAt');
  const refreshToken= event.cookies.get('refreshToken');

  console.log('Cookies on every request:', { accessToken, expiresAt, refreshToken, path: pathname });

  const isPrivate = PRIVATE_ROUTES.some(route =>
    typeof route === 'string'
      ? pathname === route
      : route.test(pathname)
  );
  if (!isPrivate) {
    if (accessToken) setSession(event, { accessToken });
    return resolve(event);
  }

  if (!accessToken && !refreshToken) {
    throw redirect(302, '/login');
  }

  const needsRefresh = !accessToken || (expiresAt && parseInt(expiresAt, 10) < Date.now());
  if (needsRefresh) {
    event.cookies.delete('accessToken', { path: '/' });
    event.cookies.delete('expiresAt',   { path: '/' });

    if (!refreshToken) {
      throw redirect(302, '/login');
    }

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

      if (res.errors || !res.data?.refresh) {
        throw new Error('Refresh failed');
      }

      accessToken = res.data.refresh.accessToken;
      const newRefreshToken = res.data.refresh.refreshToken;
      const newExpiresAt    = Date.now() + 100 * 60 * 1000;

      event.cookies.set('accessToken', accessToken, {
        path: '/', httpOnly: true, sameSite: 'strict', maxAge: 60 * 100
      });
      event.cookies.set('refreshToken', newRefreshToken, {
        path: '/', httpOnly: true, sameSite: 'strict', maxAge: 60 * 180
      });
      event.cookies.set('expiresAt', newExpiresAt.toString(), { path: '/' });
      setSession(event, { accessToken });

    } catch (err) {
      console.error('Refresh error:', err);
      event.cookies.delete('refreshToken', { path: '/' });
      event.cookies.delete('userId',       { path: '/' });
      throw redirect(302, '/login');
    }
  }

  return resolve(event);
};
