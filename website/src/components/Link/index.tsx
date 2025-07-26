import React from 'react';

import styled from '@emotion/styled';

export enum LinkMode {
  Normal,
  Small,
}

type ModeProps = {
  mode?: LinkMode;
};

export const Main = styled.a<ModeProps>(({ theme, mode = LinkMode.Normal }) => [
  {
    fontSize: 18,
    fontFamily: theme.fonts.secondary,
    color: theme.colors.basic.white,
    margin: 0,
    lineHeight: 1,
    justifyItems: 'center',
    pointerEvents: 'auto',
    cursor: 'pointer',
    fontWeight: 600,
    transition: 'text-shadow 400ms ease-in-out',

    textShadow: `0 0 5px rgba(0, 0, 0, 0)`,
    ':hover': {
      textShadow: `0 0 5px ${theme.colors.primary.highlight}`,
    },
  },
  mode === LinkMode.Normal && {
    padding: 10,
    fontSize: 18,
  },
  mode === LinkMode.Small && {
    fontSize: 10,
    padding: '0 5px',
  },
]);

type LinkProps = {
  children: string;
  onClick: () => void;
};
const Link: React.FC<LinkProps & ModeProps> = ({ children, onClick, mode }) => {
  return (
    <Main mode={mode} onClick={onClick}>
      {children}
    </Main>
  );
};

export default Link;
