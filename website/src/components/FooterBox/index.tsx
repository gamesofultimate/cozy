import React from 'react';
import styled from '@emotion/styled';

import Image from 'images/hud-background.png';

export const Main = styled.div(() => ({
  borderImageSlice: '987 212 1025 200',
  borderImageWidth: '125px 0px 125px 0px',
  borderImageOutset: '0px 0px 0px 0px',
  borderImageRepeat: 'repeat repeat',
  borderImageSource: `url(${Image})`,
  padding: '125px 0px 125px 0px',
  width: '100%',
}));

export const Inner = styled.div(({ theme }) => ({
  background: theme.colors.basic.black,
  width: '100%',
  minHeight: 300,
  padding: '40px 0',
  boxSizing: 'border-box',
}));

export const Content = styled.div(({ theme }) => ({
  background: theme.colors.basic.black,
  width: '100%',
  maxWidth: 1200,
  padding: '40px 80px',
  minHeight: 300,
  gap: 40,
  boxSizing: 'border-box',
  margin: 'auto',
}));

const Box: React.FC<React.PropsWithChildren> = ({ children }) => {
  return (
    <Main>
      <Inner>
        <Content>{children}</Content>
      </Inner>
    </Main>
  );
};

export default Box;
