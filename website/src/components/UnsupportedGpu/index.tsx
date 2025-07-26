import React from 'react';

import styled from '@emotion/styled';

import Discord from 'svgs/Discord';
import Steam from 'svgs/Steam';
import PoweredBy from 'svgs/PoweredBy';


// @ts-ignore
const Centered = styled.div(() => ({
  width: 1000,
  margin: '0 auto',
  position: 'relative',
  display: 'grid',
  gridTemplateAreas: `
    'content sidebar'
    'footer sidebar'
  `,
  gridTemplateColumns: 'minmax(0, 1fr) 0%',
  gridTemplateRows: 'minmax(0, 1fr) 100px',
  gap: '40px 80px',
}));

const Subtitle = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.primary,
  fontSize: 28,
  fontWeight: 700,
  color: '#cbe6e5',
  lineHeight: '1.6em',
  padding: '20px 0 20px',
}));

const PrimaryBody = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.primary,
  fontSize: 16,
  color: '#cbe6e5',
  lineHeight: '1.6em',
  padding: '0 0 10px',
}));

// @ts-ignore
const Content = styled.div(() => ({
  gridArea: 'content',
}));

// @ts-ignore
const Footer = styled.div(() => ({
  gridArea: 'footer',
  alignSelf: 'end',
}));

// @ts-ignore
const Continue = styled.div(({ theme }) => ({
  display: 'flex',
  gap: 40,
  padding: '40px 0 0',
  'svg': {
    transition: 'filter 400ms ease-in-out',
    '&:hover': {
      cursor: 'pointer',
      filter: `drop-shadow(0px 0px 5px ${theme.colors.primary.highlight})`,
    },
  },
}));

type UnsupportedGpuProps = {
};

const UnsupportedGpu: React.FC<UnsupportedGpuProps> = () => {

  return (
    <Centered>
      <Content>
        <Subtitle>
          Oh no! Your system isn't supported.
        </Subtitle>
        <PrimaryBody>
          Looks like your current setup doesn't meet the requirements to run the game. Here's what you can do:
          <br />
          <br />
          If you have a dedicated GPU (e.g., NVIDIA or AMD), force your browser to use it! Check your GPU settings to ensure the browser is running on the dedicated GPU.

          <div style={{ position: 'relative', paddingBottom: '62.5%', height: 0 }}>
            <iframe
              // @ts-ignore
              credentialLess="true"
              title="Instructions to force browser to use dedicated GPU"
              referrerPolicy="origin-when-cross-origin"
              src="https://www.loom.com/embed/bfaec837dbba46d98f7b8aa82e1fa214?sid=ca38dae9-99ee-4cfa-895c-9938fb5ca39b"
              frameBorder="0"
              allowFullScreen
              style={{ position: 'absolute', top: 0, left: 0, width: '100%', height: '100%' }}
            ></iframe>
          </div>
          <br />
          <br />
          If you only have an integrated GPU (e.g., Intel HD Graphics), unfortunately the game isn't compatible with your system.
          <br />
          We are improving our technology to support more configurations such as yours. Please check back soon!
        </PrimaryBody>
        <Continue>
          <a href="https://discord.gg/6XZ3S6wN" target="_blank" rel="noreferrer">
            <Discord />
          </a>
          <a href="https://store.steampowered.com/app/1933370/Mark_of_the_Deep/" target="_blank" rel="noreferrer">
            <Steam />
          </a>
        </Continue>
      </Content>
      <Footer>
        <Continue>
          <a href="https://getchaotic.com" target="_blank" rel="noreferrer">
            <PoweredBy />
          </a>
        </Continue>
      </Footer>
    </Centered >
  );
};


export default UnsupportedGpu;
