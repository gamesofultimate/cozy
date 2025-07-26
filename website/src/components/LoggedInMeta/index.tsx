import React from 'react';
import { useLocalRef } from 'hooks/useCacheState';

import Link, { LinkMode } from 'components/Link';
import styled from '@emotion/styled';

export const Main = styled.div(({ theme }) => ({
  padding: '10px 0',
  display: 'flex',
  pointerEvents: 'auto',
}));

type MenuProps = {
  onLogout?: () => void;
};

const Menu: React.FC<MenuProps> = ({ onLogout }) => {
  const [, setAccessToken] = useLocalRef<null | string>('settings.access-token', null);
  const handleLogout = () => {
    setAccessToken(null);
    onLogout?.();
  };
  return (
    <Main>
      <Link mode={LinkMode.Small} onClick={handleLogout}>
        Logout
      </Link>
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
