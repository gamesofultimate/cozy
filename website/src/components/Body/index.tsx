import React from 'react';

import styled from '@emotion/styled';

export const Main = styled.p(({ theme }) => ({
  fontSize: 13,
  fontFamily: theme.fonts.secondary,
  color: theme.colors.basic.white,
  lineHeight: '1.6em',
  fontWeight: 300,
}));

type BodyProps = {
  children: React.ReactNode;
};
const Body: React.FC<BodyProps> = ({ children }) => {
  return <Main>{children}</Main>;
};

export default Body;
