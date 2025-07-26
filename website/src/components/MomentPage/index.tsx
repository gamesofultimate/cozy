import React, { useMemo } from 'react';
import { useNavigate, useParams } from 'react-router-dom';

import Reading, { Logo, Main, Right } from 'components/Reading';

import { Result, Moment } from '@ultimate-games/canvas';
import { GetMoment } from 'types';

import Logo1x from 'images/logo@1x.png';
import Logo2x from 'images/logo@2x.png';

import styled from '@emotion/styled';

import { relative } from 'utils/datetime';
import { useNotifications } from 'hooks/useNotifications';
import Alert from 'components/Alert';
import LoggedOutMenu from 'components/LoggedOutMenu';
import Image from 'components/Image';
import { useQuery } from 'hooks/useBackend';
import CopyShare from 'svgs/CopyShare';
import Discord from 'svgs/Discord';
import PoweredBy from 'svgs/PoweredBy';

// @ts-ignore
const Centered = styled.div(() => ({
  width: '100%',
  margin: '0 auto',
  position: 'relative',
  display: 'grid',
  gridTemplateAreas: `
    'header'
    'content'
    'footer'
  `,
  gridTemplateColumns: 'minmax(0, 1fr)',
  gridTemplateRows: '100px minmax(0, 1fr) 100px',
  gap: '40px 80px',
}));

// @ts-ignore
const Header = styled.div(() => ({
  gridArea: 'header',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
}));

// @ts-ignore
const Content = styled.div(() => ({
  gridArea: 'content',
}));

// @ts-ignore
const Footer = styled.div(() => ({
  gridArea: 'footer',
  alignSelf: 'end',
  borderTop: '1px solid #2a2a2a',
}));

// @ts-ignore
const Video = styled.video(() => ({
  width: '100%',
  height: 'auto',
  boxShadow: '0 4px 150px #303030',
  objectFit: 'cover',
}));

const Title = styled.div(({ theme }) => ({
  fontSize: 32,
  fontFamily: theme.fonts.primary,
  color: theme.colors.basic.white,
  padding: 0,
  margin: 0,
  textAlign: 'center',
  lineHeight: 1,
  justifyItems: 'center',
}));

const Label = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  color: '#545454',
  fontSize: 12,
}));

// @ts-ignore
const Continue = styled.div(({ theme }) => ({
  display: 'flex',
  justifyContent: 'space-between',
  alignItems: 'center',
  gap: 40,
  padding: '20px 0 0',
  'svg': {
    transition: 'filter 400ms ease-in-out',
    '&:hover': {
      cursor: 'pointer',
      filter: `drop-shadow(0px 0px 5px ${theme.colors.primary.highlight})`,
    },
  },
}));

const MomentPage: React.FC = () => {
  const [notifications, notify] = useNotifications();
  const { moment_id } = useParams();
  const navigate = useNavigate();
  const momentGetter = useMemo((): GetMoment | null => {
    if (moment_id) return { moment_id };
    else return null;
  }, [moment_id]);
  const [, momentQuery] = useQuery<Result<Moment, any>, GetMoment | null>('/get-moment', momentGetter);

  const moment = momentQuery?.Ok ?? null;
  console.log(momentQuery);

  const handleCopyMomentUrl = (id: string) => (event: any) => {
    event.preventDefault();
    event.stopPropagation();
    const url = `${window.location.origin}/moment/${id}`;
    navigator.clipboard.writeText(url);
    notify("Url copied to your clipboard", 5000);
  }

  return (
    <Reading>
      <Logo>
        <Image onClick={() => navigate('/')} source={Logo1x} retina={Logo2x} alt="Mark of the Deep's logo" />
      </Logo>
      <Right>
        <LoggedOutMenu />
      </Right>
      <Main>
        {notifications.map((notification) => (
          <Alert key={notification.id}>{notification.content}</Alert>
        ))}
        <Centered>
          {moment && (
            <Header>
              <div style={{ textAlign: 'left', paddingTop: 15 }}>
                <Title>Moment - {moment.message ?? "Event"}</Title>
                <Label>{relative(new Date(moment.created_at))}</Label>
              </div>
              <div>
                <CopyShare onClick={handleCopyMomentUrl(moment.id)} />
              </div>
            </Header>
          )}
          <Content>
          {moment && (
            <Video autoPlay loop muted crossOrigin='anonymous'>
              <source src={moment.video_url} />
              Download the <a href={moment.video_url}>video demo</a>.
            </Video>
          )}
          </Content>
          <Footer>
            <Continue>
              <a href="https://discord.gg/6XZ3S6wN" target="_blank" rel="noreferrer">
                <Discord />
              </a>
              <a href="https://getchaotic.com" target="_blank" rel="noreferrer">
                <PoweredBy />
              </a>
            </Continue>
          </Footer>
        </Centered>
      </Main>
    </Reading>
  );
};

export default MomentPage;
