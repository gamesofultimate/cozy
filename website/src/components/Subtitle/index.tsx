import React, { ReactNode } from 'react';

import styled from '@emotion/styled';

export const Main = styled.h2(({ theme }) => ({
  fontSize: 28,
  fontFamily: theme.fonts.primary,
  color: theme.colors.basic.white,
  padding: 0,
  margin: 0,
  lineHeight: 1,
  justifyItems: 'center',
}));

type SubtitleProps = {
  children: ReactNode;
};
const Subtitle: React.FC<SubtitleProps> = ({ children }) => {
  return <Main>{children}</Main>;
};

export default Subtitle;
