import React from 'react';

import styled from '@emotion/styled';

export const Main = styled.img(({ theme, onClick }) => [
  {
    fontSize: 18,
    fontFamily: theme.fonts.secondary,
    color: theme.colors.basic.white,
    margin: 0,
    lineHeight: 1,
    justifyItems: 'center',
    pointerEvents: 'auto',
    fontWeight: 600,
    transition: 'filter 400ms ease-in-out',
    width: '100%',
  },
  onClick && {
    cursor: 'pointer',
    ':hover': {
      filter: `drop-shadow(0px 0px 5px ${theme.colors.primary.highlight})`,
    },
  },
]);

type ImageProps = {
  source: string;
  retina: string;
  alt: string;
  onClick?: () => void;
};
const Image: React.FC<ImageProps> = ({ source, retina, alt, onClick }) => {
  return <Main src={source} srcSet={`${retina} 2x`} alt={alt} onClick={onClick} />;
};

export default Image;
