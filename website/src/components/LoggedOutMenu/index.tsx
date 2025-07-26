import React from 'react';
import { useNavigate } from 'react-router-dom';
import FancyButton from 'components/FancyButton';

import styled from '@emotion/styled';
import Link from 'components/Link';

export const Main = styled.div(() => ({
  padding: '10px 0',
  width: 175,
  display: 'flex',
}));

type MenuProps = {
  onSignup?: () => void;
};

const Menu: React.FC<MenuProps> = ({ onSignup }) => {
  const navigate = useNavigate();
  const onClose = () => {
    navigate('/');
  }

  return (
    <Main>
      {window.location.pathname === '/' ? (
        <>
        {onSignup && <Link onClick={onSignup}>Sign-up</Link>}
        <Link onClick={() => navigate('/login')}>Login</Link>
        </>
      ) : (
        <FancyButton onClick={onClose}>
          Play a new session
        </FancyButton>
      )}
    </Main>
  );
};

export default Menu;
