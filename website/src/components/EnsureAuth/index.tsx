import React, { useMemo } from 'react';
import { useSearchParams, Outlet, Navigate, useLocation } from 'react-router-dom';

import { useQuery } from 'hooks/useBackend';
import { useLocalRef, useSessionRef } from 'hooks/useCacheState';
import { Result, Auth } from '@ultimate-games/canvas';
import { Access } from 'types';

type PostAuth = {
  type: 'game';
  url: string;
};

export const EnsureInvite: React.FC<React.PropsWithChildren> = () => {
  const [searchParams] = useSearchParams();
  const location = useLocation();

  // START: Grab necessary data
  const [access_token] = useLocalRef<null | string>('settings.access-token', null);
  const query = useMemo(() => {
    if (access_token) return { access_token };
    else return null;
  }, [access_token]);
  const [loadingUser, userQuery] = useQuery<Result<Auth, any>, Access | null>('/user', query);

  const [postInvite, setPostInvite] = useSessionRef<PostAuth | null>('settings.post-invite', null);
  const auth = userQuery?.Ok ?? null;
  // END: Grab necessary data

  // START: Ensure user has a proper invite token ready to go on their url
  // `info`: invite info
  /*
  useEffect(() => {
    if (!info) return;

    setSearchParams({ invite: info.invitationToken }, { replace: true });
  }, [info, setSearchParams]);
  */
  // END: Ensure user has a proper invite token ready to go on their url

  if (!auth && !loadingUser && window.location.href !== '/auth/login') {
    const userInvite = searchParams.get('invite');
    if (userInvite) {
      setPostInvite({ type: 'game', url: location.pathname });
      return <Navigate to={`/auth/invitation/${userInvite}`} />;
    }
  }

  if (postInvite) {
    setPostInvite(null);
    return <Navigate to={postInvite.url} />;
  }

  return <Outlet />;
};

/*
import { useGetUserQuery } from './queries';

export const EnsureAuth: React.FC = () => {
  const [searchParams] = useSearchParams();
  const userQuery = useGetUserQuery();
  const location = useLocation();
  const [postAuth, setPostAuth] = useSessionRef<PostAuth | null>('settings.post-auth', null);
  const auth = userQuery.data?.auth ?? null;

  if (!auth && !userQuery.loading && window.location.href !== '/auth/login') {
    const userInvite = searchParams.get('invite');
    if (!userInvite) {
      setPostAuth({ type: 'game', url: location.pathname });
      return <Navigate to="/auth/login" />;
    }
  }

  if (postAuth) {
    setPostAuth(null);
    return <Navigate to={postAuth.url} />;
  }

  return <Outlet />;
};
 */
