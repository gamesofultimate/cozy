import React from 'react';
import { useLocalRef } from 'hooks/useCacheState';
import { useGameData } from 'data/game';

import Link, { LinkMode } from 'components/Link';
import styled from '@emotion/styled';
import Eyebrow, { EyebrowMode } from 'components/Eyebrow';

export const Main = styled.div(({ theme }) => ({
  padding: '10px 0',
  display: 'flex',
  pointerEvents: 'auto',
  justifyContent: 'space-between',
  width: 300,
}));

export const Row = styled.div(({ theme }) => ({
  display: 'flex',
  justifyContent: 'space-between',
}));

type MenuProps = {
  username: string | null;
  onLogout?: () => void;
};

const Menu: React.FC<MenuProps> = ({ username, onLogout }) => {
  const game = useGameData();
  const [, setAccessToken] = useLocalRef<null | string>('settings.access-token', null);
  const handleLogout = () => {
    setAccessToken(null);
    onLogout?.();
  };
  return (
    <Main>
      <div>
        <Eyebrow mode={EyebrowMode.Secondary}>
          <span>Welcome back,</span>
        </Eyebrow>
        <Eyebrow>{username ? <strong>{username}</strong> : <strong>Unknown</strong>}</Eyebrow>
      </div>
      <div>
        <Link mode={LinkMode.Small} onClick={() => game.openInvitationDialog()}>
          Invite
        </Link>
        <Link mode={LinkMode.Small} onClick={handleLogout}>
          Logout
        </Link>
      </div>
    </Main>
  );
};

/*
    <Main>
      <Button onClick={onSignup}>Sign-up</Button>
      <Link onClick={() => navigate('/login')}>Login</Link>
    </Main>
   */

export default Menu;
