import React, { useMemo } from 'react';

import styled from '@emotion/styled';

import { GetMoments, GetAchievements } from 'types';
import { Result, Moment, Achievement } from '@ultimate-games/canvas';
import { useQuery } from 'hooks/useBackend';
import { useNotifications } from 'hooks/useNotifications';
import { relative } from 'utils/datetime';
import Discord from 'svgs/Discord';
import Steam from 'svgs/Steam';
import PoweredBy from 'svgs/PoweredBy';
import Link from 'svgs/Link';

import Coin from 'svgs/Coin';
import Demo from 'videos/demo.mp4';
import FancyButton from 'components/FancyButton';
import Alert from 'components/Alert';
import CopyShare from 'svgs/CopyShare';
import TiktokShare from 'svgs/TiktokShare';
import { useGameData } from 'data/game';

// @ts-ignore
const Centered = styled.div(() => ({
  width: 1200,
  height: 'calc(100vh - 350px)',
  margin: '0 auto',
  position: 'relative',
  display: 'grid',
  gridTemplateAreas: `
    'header moments sidebar'
    'content moments sidebar'
    'achievements achievements achievements'
    'footer footer footer'
  `,
  gridTemplateColumns: 'minmax(0, 1fr) 300px 30%',
  gridTemplateRows: '60px minmax(0, 1fr) 60px 100px',
  gap: '20px 40px',
}));

// @ts-ignore
const Header = styled.div(() => ({
  gridArea: 'header',
  display: 'flex',
  flexWrap: 'wrap',
  gap: 30,
}));

// @ts-ignore
const Moments = styled.div(() => ({
  gridArea: 'moments',
  overflowY: 'scroll',
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

const Subtitle = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.primary,
  fontSize: 28,
  fontWeight: 700,
  color: '#cbe6e5',
  lineHeight: '1.6em',
}));

const PrimaryBody = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.primary,
  fontSize: 16,
  color: '#cbe6e5',
  lineHeight: '1.6em',
  padding: '0 0 10px',
}));

const SecondaryBody = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.primary,
  fontSize: 14,
  color: '#7dc0bd',
  lineHeight: '1.6em',
  padding: '10px 0 20px',
}));

// @ts-ignore
const Sidebar = styled.div(() => ({
  gridArea: 'sidebar',
  alignSelf: 'center',
  textAlign: 'center',
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
const ItemHeader = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  fontSize: 14,
  fontWeight: 900,
  color: '#348984',
}));

// @ts-ignore
const ItemValue = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.primary,
  fontSize: 24,
  color: '#CBE6E5',
  fontWeight: 700,
  textShadow: '0 2px 16px #66DFDB',
}));

const Achievements = styled.div(({ theme }) => ({
  gridArea: 'achievements',
  display: 'flex',
  flexWrap: 'wrap',
  justifyContent: 'center',
  gap: 20,
  overflowX: 'scroll',
  borderTop: '1px solid #2a2a2a',
  padding: '10px 0',
}));

const AchievementCard = styled.div(({ theme }) => ({
  textAlign: 'left',

  display: 'grid',
  gridTemplateAreas: `
    'icon header'
    'icon subheader'
  `,
  gridTemplateColumns: '72px minmax(0, 1fr)',
  gridTemplateRows: '30px 30px',
  gap: '5x',
}));

const AchievementIcon = styled.div(({ theme }) => ({
  gridArea: 'icon',
  '& svg': {
    marginTop: '-15px',
    marginLeft: '-15px',
  }
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

// @ts-ignore
const Video = styled.video(() => ({
  width: '100%',
  height: 'auto',
  boxShadow: '0 4px 150px #303030',
  clipPath: `url(#video-clip-path)`,
  maxHeight: 230,
}));

// @ts-ignore
const Input = styled.input(({ theme }) => ({
  width: '100%',
  background: '#181818',
  color: '#CBE6E5',
  fontFamily: theme.fonts.primary,
  border: 0,
  textAlign: 'center',
  fontSize: 16,
  boxSizing: 'border-box',
  padding: 15,
}));

// @ts-ignore
const Card = styled.div(({ theme }) => ({
  padding: 20,
  margin: '20px 0',
  background: '#000',
  cursor: 'pointer',
  transition: 'background 400ms ease-in-out',

  '&:hover': {
    background: '#0f0f0f',
  }
}));

// @ts-ignore
const Body = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  color: '#cbe6e5',
  fontSize: 14,
  fontWeight: 700,
}));

const Label = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  color: '#545454',
  fontSize: 12,
}));

const Hr = styled.hr(({ theme }) => ({
  display: 'block',
  height: '1px',
  border: '0',
  margin: '1em 0',
  padding: '0',
  background: '#292929',
}));

const Row = styled.div(() => ({
  display: 'flex',
  justifyContent: 'space-between',
  alignItems: 'center',
}));

type WishlistProps = {
  kills: number;
  deaths: number;
  time: string;
  unique_id: string;
  onClose: () => void;
};

const isTiktokEnabled = false;

const Wishlist: React.FC<WishlistProps> = ({ kills, deaths, time, unique_id, onClose }) => {
  const game = useGameData();

  const [notifications, notify] = useNotifications();
  const handleCopyUrl = () => {
    navigator.clipboard.writeText(`${window.location.href}results/${unique_id}`);
    notify("Url copied to your clipboard", 5000);

    game.sendShareEvent();
  };

  const momentsGetter = useMemo((): GetMoments | null => {
    if (unique_id) return { unique_id };
    else return null;
  }, [unique_id]);
  const [, momentsQuery] = useQuery<Result<Moment[], any>, GetMoments | null>('/get-moments', momentsGetter);
  const moments = momentsQuery?.Ok ?? [];

  const achievementsConfig = useMemo((): GetAchievements | null => {
    if (!unique_id) return null;
    return { unique_id: unique_id };
  }, [unique_id]);
  const [, achievementsQuery] = useQuery<Result<Achievement[], any>, GetAchievements | null>('/get-achievements', achievementsConfig);
  let achievements = achievementsQuery?.Ok ?? [];
  achievements = achievements.slice(Math.max(achievements.length - 5, 0));

  const handleCopyMomentUrl = (id: string) => (event: any) => {
    event.preventDefault();
    event.stopPropagation();
    const url = `${window.location.origin}/moment/${id}`;
    navigator.clipboard.writeText(url);
    notify("Url copied to your clipboard", 5000);
  }

  const handleTiktok = (id: string) => () => {
    console.log('tiktok');
  };

  const handleNavigateToMoment = (id: string) => () => {
    window.open(`/moment/${id}`, '_blank');
  };

  return (
    <Centered>
      <Header>
        <div>
          <ItemHeader>Kills</ItemHeader>
          <ItemValue>{kills}</ItemValue>
        </div>
        <div>
          <ItemHeader>Deaths</ItemHeader>
          <ItemValue>{deaths}</ItemValue>
        </div>
        <div>
          <ItemHeader>Time</ItemHeader>
          <ItemValue>{time}</ItemValue>
        </div>
      </Header>
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
      <Content>
        <Subtitle>
          Thank you for playing!
        </Subtitle>
        <PrimaryBody>
          You still have more areas to explore, formidable bosses to defeat and unforgettable characters to meet! Your journey has only just begun! Continue the adventure on Steam now!</PrimaryBody>
        <Continue>
          <a href="https://store.steampowered.com/app/1933370/Mark_of_the_Deep/" target="_blank" rel="noreferrer">
            <Steam />
          </a>
        </Continue>
      </Content>
      <Footer>
        <Continue>
          <FancyButton onClick={onClose}>
            Play again
          </FancyButton>
          <a href="https://discord.gg/6XZ3S6wN" target="_blank" rel="noreferrer">
            <Discord />
          </a>
          <a href="https://getchaotic.com" target="_blank" rel="noreferrer">
            <PoweredBy />
          </a>
        </Continue>
      </Footer>
      <Moments>
        <Subtitle>Moments</Subtitle>
        <div>
          {moments.map((moment) => (
            <Card key={moment.id} onClick={handleNavigateToMoment(moment.id)}>
              <Video autoPlay loop muted crossOrigin='anonymous'>
                <source src={moment.video_url} />
                Download the <a href={moment.video_url}>video demo</a>.
              </Video>
              <div style={{ textAlign: 'left', paddingTop: 15 }}>
                <Body>{moment.message ?? "Event"}</Body>
                <Label>{relative(new Date(moment.created_at))}</Label>
              </div>
              <Hr />
              <Row>
                {isTiktokEnabled ? <TiktokShare onClick={handleTiktok(moment.id)} /> : <div />}
                <CopyShare onClick={handleCopyMomentUrl(moment.id)} />
              </Row>
            </Card>
          ))}
        </div>
      </Moments>
      <Sidebar>
        <svg width="0" height="0" viewBox="0 0 1 1" fill="none" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <clipPath id="video-clip-path" clipPathUnits="objectBoundingBox">
              <path d="M0.830556 0.0633333L1 0.0166667V0.933333L0.711111 0.993333L0.408333 0.95L0.372222 1L0.175 0.94L0 0.993333V0.0266667L0.577778 0.0466667L0.619444 0L0.830556 0.0633333Z" fill="black" />
            </clipPath>
          </defs>
        </svg>
        <Video autoPlay loop muted crossOrigin='anonymous'>
          <source src={Demo} type="video/mp4" />
          Download the <a href={Demo}>video demo</a>.
        </Video>
        <Subtitle>
          Share your results
        </Subtitle>
        <SecondaryBody>Use this URL to share your results with your friends!</SecondaryBody>
        <Input value={`${window.location.href}results/${unique_id}`} />
        <FancyButton onClick={handleCopyUrl}>
          <span>
            <Link />Copy Url
          </span>
        </FancyButton>
        {notifications.map((notification) => (
          <Alert key={notification.id}>{notification.content}</Alert>
        ))}
      </Sidebar>
    </Centered >
  );
};


export default Wishlist;
