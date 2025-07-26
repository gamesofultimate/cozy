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
  border: '1px solid #df9215',
  transition: 'padding 200ms ease-in-out, border-color 200ms ease-in-out, box-shadow 200ms ease-in-out',
  boxShadow: '0 0 30px rgba(255, 255, 255, .3)',

  '&:hover': {
    padding: 0,
    border: '1px solid #df9215',
  },
}));

// @ts-ignore
const Inner = styled.div(({ theme }) => ({
  background: 'linear-gradient(90deg, rgba(176,78,33,1) 0%, rgba(220,117,38,1) 100%)',
  fontSize: 32,
  padding: '10px 20px',
  textTransform: 'uppercase',
}));

// @ts-ignore
const Main = styled.div(({ theme }) => ({
  display: 'block',
  color: '#CBE6E5',
  fontFamily: theme.fonts.primary,
  textAlign: 'center',
  boxSizing: 'border-box',
  cursor: 'pointer',
  background: 'transparent',
  padding: 0,

  transition: 'padding 200ms ease-in-out',

  '&:hover': {
    padding: 5,

    // @ts-ignore
    [GrowingRoom]: {
      padding: 0,
      border: '1px solid transparent',
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
