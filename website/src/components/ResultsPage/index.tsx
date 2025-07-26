import React, { useMemo } from 'react';
import { useNavigate, useParams } from 'react-router-dom';

import Reading, { Logo, Main, Right } from 'components/Reading';

import { Result, PlaySession, Achievement } from '@ultimate-games/canvas';
import { GetAchievements, GetPlaySession } from 'types';

import Logo1x from 'images/logo@1x.png';
import Logo2x from 'images/logo@2x.png';

import styled from '@emotion/styled';

import { relative } from 'utils/datetime';
import Coin from 'svgs/Coin';
import Discord from 'svgs/Discord';
import PoweredBy from 'svgs/PoweredBy';
import FancyButton from 'components/FancyButton';
import LoggedOutMenu from 'components/LoggedOutMenu';
import Image from 'components/Image';
import { useQuery } from 'hooks/useBackend';

// @ts-ignore
const Centered = styled.div(() => ({
  width: '100%',
  margin: '0 auto',
  position: 'relative',
  display: 'grid',
  gridTemplateAreas: `
    'header header'
    'content content'
    'footer footer'
  `,
  gridTemplateColumns: 'minmax(0, 1fr) 300px',
  gridTemplateRows: '160px minmax(0, 1fr) 100px',
  gap: '40px 80px',
}));

// @ts-ignore
const Header = styled.div(() => ({
  gridArea: 'header',
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

// @ts-ignore
const Stats = styled.div(() => ({
  display: 'flex',
  flexWrap: 'wrap',
  gap: 30,
  paddingTop: 20,
  justifyContent: 'center',
}));

// @ts-ignore
const ItemHeader = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  fontSize: 14,
  fontWeight: 900,
  color: '#348984',
  textTransform: 'uppercase',
}));

// @ts-ignore
const ItemValue = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.primary,
  fontSize: 32,
  color: '#CBE6E5',
  fontWeight: 700,
  textShadow: '0 2px 16px #66DFDB',
}));

// @ts-ignore
const Video = styled.video(() => ({
  width: '100%',
  height: 'auto',
  boxShadow: '0 4px 150px #66dfdba1',
  objectFit: 'cover',
}));

const Title = styled.div(({ theme }) => ({
  fontSize: 42,
  fontFamily: theme.fonts.primary,
  color: theme.colors.basic.white,
  padding: 0,
  margin: 0,
  lineHeight: 1,
  justifyItems: 'center',
}));

const Achievements = styled.div(({ theme }) => ({
  display: 'flex',
  flexWrap: 'wrap',
  justifyContent: 'center',
  gap: 20,
  paddingTop: 20,
}));

const AchievementCard = styled.div(({ theme }) => ({
  textAlign: 'left',

  display: 'grid',
  gridTemplateAreas: `
    'icon header'
    'icon subheader'
  `,
  gridTemplateColumns: '72px minmax(0, 1fr)',
  gridTemplateRows: '1fr 1fr',
  gap: '5x',
}));

const AchievementIcon = styled.div(({ theme }) => ({
  gridArea: 'icon',
}));

const AchievementHeader = styled.div(({ theme }) => ({
  gridArea: 'header',
  alignSelf: 'end',
}));

const AchievementSubheader = styled.div(({ theme }) => ({
  gridArea: 'subheader',
  alignSelf: 'start',
}));

const AchieveTitle = styled.div(({ theme }) => ({
  fontSize: 22,
  fontFamily: theme.fonts.primary,
  color: '#42a7a1',
  paddingBottom: 5,
  margin: 0,
  lineHeight: 1,
  justifyItems: 'center',
}));

const AchieveBody = styled.div(({ theme }) => ({
  fontSize: 18,
  fontFamily: theme.fonts.primary,
  color: '#9b9b9b',
  padding: 0,
  margin: 0,
  lineHeight: 1,
  justifyItems: 'center',
}));

const Label = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  color: '#cbe6e5',
  fontSize: 12,
}));

const ResultsPage: React.FC = () => {
  const { session_id } = useParams();
  const navigate = useNavigate();
  const sessionGetter = useMemo((): GetPlaySession | null => {
    if (session_id) return { session_id };
    else return null;
  }, [session_id]);
  const [, sessionQuery] = useQuery<Result<PlaySession, any>, GetPlaySession | null>('/get-play-session', sessionGetter);

  const session = sessionQuery?.Ok ?? null;

  const achievementsConfig = useMemo((): GetAchievements | null => {
    if (!session?.unique_id) return null;
    return { unique_id: session.unique_id };
  }, [session]);
  const [, achievementsQuery] = useQuery<Result<Achievement[], any>, GetAchievements | null>('/get-achievements', achievementsConfig);
  const achievements = achievementsQuery?.Ok ?? [];

  const onClose = () => {
    navigate('/');
  }

  console.log(sessionQuery, achievementsQuery);

  const stats = session?.data ? JSON.parse(session.data) : {};

  return (
    <Reading>
      <Logo>
        <Image onClick={() => navigate('/')} source={Logo1x} retina={Logo2x} alt="Mark of the Deep's logo" />
      </Logo>
      <Right>
        <LoggedOutMenu />
      </Right>
      <Main>
        <Centered>
          <Header>
            <Title>Results</Title>
            <Stats>
              <div>
                <ItemHeader>Kills</ItemHeader>
                <ItemValue>{stats.kills ?? 0}</ItemValue>
              </div>
              <div>
                <ItemHeader>Deaths</ItemHeader>
                <ItemValue>{stats.deaths ?? 0}</ItemValue>
              </div>
              <div>
                <ItemHeader>Time</ItemHeader>
                <ItemValue>{stats.time ?? '00:00:00'}</ItemValue>
              </div>
            </Stats>
          </Header>
          <Content>
            <div>
              {session && (
                <>
                  <Video autoPlay muted controls crossOrigin='anonymous'>
                    <source src={session.video_url} />
                    Download the <a href={session.video_url}>video demo</a>.
                  </Video>
                  <Label>{relative(new Date(session.created_at))}</Label>
                </>
              )}
            </div>
            <div style={{ textAlign: 'center', padding: '40px 0' }}>
              <Title>Achievements</Title>
              <Achievements>
                {achievements.map(achievement => (
                  <AchievementCard key={achievement.id}>
                    <AchievementIcon>
                      <Coin />
                    </AchievementIcon>
                    <AchievementHeader>
                      <AchieveTitle>{achievement.title}</AchieveTitle>
                    </AchievementHeader>
                    <AchievementSubheader>
                      <AchieveBody>{achievement.description}</AchieveBody>
                    </AchievementSubheader>
                  </AchievementCard>
                ))}
              </Achievements>
            </div>
          </Content>
          <Footer>
            <Continue>
              <FancyButton onClick={onClose}>
                Play a new session
              </FancyButton>
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

export default ResultsPage;
