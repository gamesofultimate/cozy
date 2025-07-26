import React, { useMemo } from 'react';
import { observer } from 'mobx-react';

import styled from '@emotion/styled';

import { GetMoments } from 'types';
import { Result, Moment } from '@ultimate-games/canvas';
import { useQuery } from 'hooks/useBackend';
import { relative } from 'utils/datetime';
import Discord from 'svgs/Discord';
//import Steam from 'svgs/Steam';
import PoweredBy from 'svgs/PoweredBy';

import FancyButton from 'components/FancyButton';
import CopyShare from 'svgs/CopyShare';
import TiktokShare from 'svgs/TiktokShare';

import { useGameData } from 'data/game';
import { Joystick, XBoxA, XBoxB, XBoxX, XBoxY, XBoxRB, XBoxRT, XBoxStart } from 'svgs/Controller';

// @ts-ignore
const Centered = styled.div(() => ({
  width: 1200,
  height: 'calc(100vh - 350px)',
  margin: '0 auto',
  position: 'relative',
  display: 'grid',
  gridTemplateAreas: `
    'content moments'
    'footer footer'
  `,
  gridTemplateColumns: 'minmax(0, 1fr) 300px',
  gridTemplateRows: 'minmax(0, 1fr) 100px',
  gap: '40px 80px',
}));


// @ts-ignore
const Moments = styled.div(() => ({
  gridArea: 'moments',
  overflowY: 'scroll',
}));

// @ts-ignore
const Title = styled.div(({ theme }) => ({
  fontWeight: 700,
  fontFamily: theme.fonts.primary,
  color: '#cbe6e5',
  fontSize: 36,
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

const Subtitle = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.primary,
  fontSize: 36,
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
const Table = styled.table(({ theme }) => ({
  width: '100%',
}));

// @ts-ignore
const Th = styled.td(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  fontSize: 16,
  color: '#CBE6E5',
}));

// @ts-ignore
const Td = styled.td(({ theme }) => ({
  fontFamily: theme.fonts.primary,
  fontSize: 24,
  color: '#CBE6E5',
  textShadow: '0 2px 16px #66DFDB',
}));

// @ts-ignore
const TdCentered = styled.td(({ theme }) => ({
  fontFamily: theme.fonts.primary,
  fontSize: 24,
  color: '#CBE6E5',
  textShadow: '0 2px 16px #66DFDB',
  textAlign: 'center',
  paddingRight: 15,
  width: 75,
}));

// @ts-ignore
const Video = styled.video(() => ({
  width: '100%',
  height: 'auto',
  boxShadow: '0 4px 150px #303030',
}));

type PauseProps = {
  onClose: () => void;
  unique_id: string;
};

const isTiktokEnabled = false;

const Pause: React.FC<PauseProps> = ({ onClose, unique_id }) => {
  const game = useGameData();
  const momentsGetter = useMemo((): GetMoments | null => {
    if (unique_id) return { unique_id };
    else return null;
  }, [unique_id]);
  const [, momentsQuery] = useQuery<Result<Moment[], any>, GetMoments | null>('/get-moments', momentsGetter);
  const moments = momentsQuery?.Ok ?? [];
  const handleCopyMomentUrl = (id: string) => (event: any) => {
    event.preventDefault();
    event.stopPropagation();
    const url = `${window.location.origin}/moment/${id}`;
    navigator.clipboard.writeText(url);

    game.sendShareEvent();
  }

  const handleTiktok = (_: string) => () => {
    console.log('tiktok');
  };

  const handleNavigateToMoment = (id: string) => () => {
    window.open(`/moment/${id}`, '_blank');
  };

  return (
    <Centered>
      <Content>
        <Subtitle>
          Controls
        </Subtitle>
        <PrimaryBody>
          <Table>
            <tr>
              <Th>Controller</Th>
              <Th>Keyboard</Th>
              <Th>Action</Th>
            </tr>
            <tr>
              <TdCentered><Joystick /></TdCentered>
              <Td>WSAD</Td>
              <Td>Move</Td>
            </tr>
            <tr>
              <TdCentered><XBoxA /></TdCentered>
              <Td>Spacebar</Td>
              <Td>Roll</Td>
            </tr>
            <tr>
              <TdCentered><XBoxX /></TdCentered>
              <Td>Left Mouse</Td>
              <Td>Attack</Td>
            </tr>
            {game.hasGun && (
              <tr>
                <TdCentered><XBoxB /></TdCentered>
                <Td>Right Mouse</Td>
                <Td>Pistol</Td>
              </tr>
            )}
            <tr>
              <TdCentered><XBoxY /></TdCentered>
              <Td>Q</Td>
              <Td>Potion</Td>
            </tr>
            <tr>
              <TdCentered><XBoxRB /></TdCentered>
              <Td>F</Td>
              <Td>Interact</Td>
            </tr>
            {game.hasHookshot && (
              <tr>
                <TdCentered><XBoxRT /></TdCentered>
                <Td>E</Td>
                <Td>Hookshot</Td>
              </tr>
            )}
            <tr>
              <TdCentered><XBoxStart /></TdCentered>
              <Td>Escape</Td>
              <Td>Pause</Td>
            </tr>
          </Table>
        </PrimaryBody>
        {/*
        <a href="https://store.steampowered.com/app/1933370/Mark_of_the_Deep/" target="_blank" rel="noreferrer">
          <Steam />
        </a>
        */}
      </Content>
      <Moments>
        <Title>Moments</Title>
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
      <Footer>
        <Continue>
          <FancyButton onClick={onClose}>
            Close
          </FancyButton>
          <a href="https://discord.gg/6XZ3S6wN" target="_blank" rel="noreferrer">
            <Discord />
          </a>
          <a href="https://getchaotic.com" target="_blank" rel="noreferrer">
            <PoweredBy />
          </a>
        </Continue>
      </Footer>
    </Centered >
  );
};


export default observer(Pause);
