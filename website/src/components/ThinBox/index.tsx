import React from 'react';
import styled from '@emotion/styled';

export const Main = styled.div(() => ({
  padding: '125px 150px 125px 150px',
}));

// @ts-ignore
export const Inner = styled.div(() => ({
  background: '#01070f',
  display: 'flex',
  padding: '40px',
  minHeight: 300,
  gap: 80,
  boxSizing: 'border-box',
  pointerEvents: 'auto',
}));

const Box: React.FC<React.PropsWithChildren> = ({ children }) => {
  return (
    <Main>
      <Inner>{children}</Inner>
    </Main>
  );
};

export default Box;
