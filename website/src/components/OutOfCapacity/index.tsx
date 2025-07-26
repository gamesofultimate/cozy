import React from 'react';

import styled from '@emotion/styled';

import { Discord } from 'svgs/SocialMedia';
import PoweredBy from 'svgs/PoweredBy';
import Button from 'components/Button';

// @ts-ignore
const Centered = styled.div(() => ({
  width: 1000,
  margin: '0 auto',
  position: 'relative',
  display: 'grid',
  gridTemplateAreas: `
    'content'
    'footer'
  `,
  gridTemplateColumns: 'minmax(0, 1fr)',
  gridTemplateRows: 'minmax(0, 1fr) 100px',
  gap: '40px 80px',
}));

const Subtitle = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.primary,
  fontSize: 28,
  fontWeight: 700,
  color: '#fff',
  lineHeight: '1.6em',
  padding: '20px 0 20px',
}));

const PrimaryBody = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  fontSize: 16,
  color: '#aaa',
  lineHeight: '1.6em',
  padding: '0 0 10px',
}));

// @ts-ignore
const Content = styled.div(() => ({
  gridArea: 'content',
  textAlign: 'center',
  display: 'flex',
  flexDirection: 'column',
  justifyContent: 'center',
  alignItems: 'center',
}));

// @ts-ignore
const Footer = styled.div(() => ({
  gridArea: 'footer',
  display: 'flex',
  alignSelf: 'end',
  justifyContent: 'center',
  textAlign: 'center',
}));

// @ts-ignore
const GetChaos = styled.a(() => ({
}));

// @ts-ignore
const Continue = styled.div(({ theme }) => ({
  display: 'flex',
  gap: 40,
  padding: '40px 0 0',
  textAlign: 'center',
  alignItems: 'center',

  // @ts-ignore
  [GetChaos]: {
    color: '#fff',
    textDecoration: 'none',
    fontFamily: theme.fonts.secondary,
    alignItems: 'center',
    verticalAlign: 'center',
    display: 'flex',
    gap: 15,
  },
  'svg': {
    transition: 'filter 400ms ease-in-out',
    '&:hover': {
      cursor: 'pointer',
      filter: `drop-shadow(0px 0px 5px #b04e21)`,
    },
  },
}));

type OutOfCapacityProps = {
  onClose: () => void;
};

const OutOfCapacity: React.FC<OutOfCapacityProps> = ({ onClose }) => {
  return (
    <Centered>
      <Content>
        <Subtitle>
          Alert
        </Subtitle>
        <PrimaryBody>
          Unfortunately our servers are at capacity right now.
          <br />
          We're working hard to get more servers up and running, so please check back soon.
        </PrimaryBody>
        <Continue>
          <Button href="https://discord.gg/6XZ3S6wN" target="_blank" rel="noreferrer">
            <>
              <Discord />
              <span>Join our Discord</span>
            </>
          </Button>
        </Continue>
      </Content>
      <Footer>
        <Continue>
          <GetChaos href="https://getchaotic.com" target="_blank" rel="noreferrer">
            <PoweredBy />
          </GetChaos>
        </Continue>
      </Footer>
    </Centered >
  );
};


export default OutOfCapacity;
