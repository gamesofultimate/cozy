import React from 'react';

import styled from '@emotion/styled';

export enum ButtonKind {
  Normal,
  Large,
}

type KindProps = {
  kind?: ButtonKind;
};


// @ts-ignore
const GrowingRoom = styled.div(({ theme }) => ({
  padding: 5,
  //background: '#8CDD4B',
  transition: 'padding 200ms ease-in-out, border-color 200ms ease-in-out, box-shadow 200ms ease-in-out',
  boxShadow: '0 0 30px rgba(255, 255, 255, .7)',
  borderRadius: 18,

  '&:hover': {
    padding: 0,
    border: '1px solid #3DA743',
  },
}));

// @ts-ignore
const Inner = styled.div(({ theme }) => ({
  background: '#8CDD4B',
  fontSize: 32,
  fontWeight: 900,
  padding: '10px 20px',
  textTransform: 'uppercase',
  borderRadius: 16,
}));

// @ts-ignore
const Main = styled.div(({ theme }) => ({
  display: 'inline-block',
  color: '#5B5B5B',
  fontFamily: theme.fonts.secondary,
  textAlign: 'center',
  boxSizing: 'border-box',
  cursor: 'pointer',
  background: 'transparent',
  padding: 0,
  maxWidth: 200,

  transition: 'padding 200ms ease-in-out',

  '&:hover': {
    padding: 5,

    // @ts-ignore
    [GrowingRoom]: {
      padding: 0,
      border: '1px solid #3DA743',
      boxShadow: '0 0 0 rgba(255, 255, 255, .0)',
    },
  }
}));

type ButtonProps = {
  children: any;
  onClick?: (event: any) => void;
  submit?: boolean;
};
const Button: React.FC<ButtonProps & KindProps> = ({ children, submit, kind, onClick }) => {
  return (
    <Main onClick={onClick}>
      <GrowingRoom>
        <Inner>{children}</Inner>
      </GrowingRoom>
    </Main>
  );
};

export default Button;
