import React from 'react';

import styled from '@emotion/styled';

export enum ProgressBarKind {
  Normal,
  Large,
}

type KindProps = {
  kind?: ProgressBarKind;
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

type InnerProps = {
  percent: number;
};

// @ts-ignore
const Inner = styled.div<InnerProps>(({ theme, percent = 0 }) => ({
  background: '#8CDD4B',
  fontSize: 32,
  fontWeight: 900,
  padding: '10px 0px',
  textTransform: 'uppercase',
  borderRadius: 16,
  width: `${100 * percent}%`,
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
  width: '100%',

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

type ProgressBarProps = {
  percent: number;
};
const ProgressBar: React.FC<ProgressBarProps & KindProps> = ({ percent }) => {
  return (
    <Main>
      <GrowingRoom>
        <Inner percent={percent} />
      </GrowingRoom>
    </Main>
  );
};

export default ProgressBar;
