import React from 'react';

import styled from '@emotion/styled';
import Subtitle from 'components/Subtitle';
import Body from 'components/Body';
import Image from 'components/Image';

// @ts-ignore
export const Main = styled.div(() => ({
  position: 'relative',
  display: 'grid',
  justifyItems: 'center',
  gridAutoRows: 'minmax(0, 1fr) 30px minmax(0, 1fr)',
  gridTemplateAreas: `
    "image"
    "title"
    "description"
  `,
}));

// @ts-ignore
export const Gradient = styled.div(({ theme }) => ({
  position: 'absolute',
  background: `radial-gradient(circle, ${theme.colors.background.mutedAccent} 0%, rgba(0, 0, 0, 0) 60%)`,
  width: 150,
  height: 150,
  top: -40,
  zIndex: 5,
}));

export const ImageBlock = styled.div(() => ({
  gridArea: 'image',
  zIndex: 10,
}));

export const Title = styled.div(() => ({
  gridArea: 'title',
  zIndex: 10,
}));

// @ts-ignore
export const Description = styled.div(() => ({
  gridArea: 'description',
  padding: 10,
  textAlign: 'center',
  zIndex: 10,
}));

type InfoBlockProps = {
  title: string;
  description: string;
  image: {
    main: string;
    retina: string;
    alt: string;
  };
};
const InfoBlock: React.FC<InfoBlockProps> = ({ title, image, description }) => {
  return (
    <Main>
      <ImageBlock>
        <Image source={image.main} retina={image.retina} alt={image.alt} />
      </ImageBlock>
      <Title>
        <Subtitle>{title}</Subtitle>
      </Title>
      <Description>
        <Body>{description}</Body>
      </Description>
    </Main>
  );
};

export default InfoBlock;
