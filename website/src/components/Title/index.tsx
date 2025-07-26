import React from 'react';

import styled from '@emotion/styled';

export const Main = styled.h1(({ theme }) => ({
  fontSize: 32,
  fontFamily: theme.fonts.primary,
  color: theme.colors.basic.white,
  padding: 0,
  margin: 0,
  textAlign: 'center',
  lineHeight: 1,
  justifyItems: 'center',
}));

type TitleProps = {
  children: string;
};
const Title: React.FC<TitleProps> = ({ children }) => {
  return <Main>{children}</Main>;
};

export default Title;
