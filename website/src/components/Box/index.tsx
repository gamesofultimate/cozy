import React from 'react';
import styled from '@emotion/styled';

import Image from 'images/3-box-border.png';

const PADDING_HORIZONTAL = 200;
const PADDING_VERTICAL = 150;

export const Main = styled.div(() => ({
  borderImageSlice: '541 416 525 516',
  borderImageWidth: `${PADDING_VERTICAL}px ${PADDING_HORIZONTAL}px ${PADDING_VERTICAL}px ${PADDING_HORIZONTAL}px`,
  borderImageOutset: '0px 0px 0px 0px',
  borderImageRepeat: 'repeat repeat',
  borderImageSource: `url(${Image})`,
  padding: '125px 150px 125px 150px',
  width: '60%',
  maxWidth: 1200,
}));

export const Inner = styled.div(({ theme }) => ({
  background: theme.colors.background.box,
  display: 'flex',
  width: '100%',
  padding: '40px 80px',
  minHeight: 300,
  gap: 40,
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
