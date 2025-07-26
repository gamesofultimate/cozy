import React, { useMemo } from 'react';
//import { useSearchParams } from 'react-router-dom';

import styled from '@emotion/styled';
import { Result, Moment, Ranking, PlayerRank } from '@ultimate-games/canvas';
import { GetMoments, GetRanking } from 'types';

import PoweredBy from 'svgs/PoweredBy';
import { Discord } from 'svgs/SocialMedia';
import { useNotifications } from 'hooks/useNotifications';
import Alert from 'components/Alert';
import { useQuery } from 'hooks/useBackend';
import { relative } from 'utils/datetime';
import FancyButton from 'components/FancyButton';
import CopyShare from 'svgs/CopyShare';
import TiktokShare from 'svgs/TiktokShare';
import Sidebar from 'svgs/Sidebar';
import { useGameData } from 'data/game';

// @ts-ignore
export const Main = styled.div(() => ({
  position: 'relative',
  display: 'grid',
  gridTemplateAreas: `
    'content'
    'stage'
    'footer'
  `,
  gridTemplateColumns: 'minmax(0, 1fr)',
  gridTemplateRows: '130px minmax(0, 1fr) 100px',
  gap: '40px 80px',
  paddingTop: 40,
}));

const Content = styled.div(() => ({
  gridArea: 'content',
  margin: '0 auto',
  width: '100%',
  zIndex: 10,
}));

// @ts-ignore
const Stage = styled.div(() => ({
  gridArea: 'stage',
  margin: '0 auto',
  width: 400,
  overflowY: 'scroll',
  maxHeight: '50vh',
  zIndex: 10,
}));

const Footer = styled.div(() => ({
  gridArea: 'footer',
  margin: '0 auto',
  width: 400,
  zIndex: 10,
}));

const Row = styled.div(() => ({
  display: 'flex',
  justifyContent: 'space-between',
  alignItems: 'center',
}));

// @ts-ignore
const Video = styled.video(() => ({
  width: '100%',
  height: 'auto',
  boxShadow: '0 4px 150px #303030',
  maxHeight: 250,
  objectFit: 'cover',
  clipPath: `url(#video-clip-path)`,
}));


// @ts-ignore
const GetChaos = styled.a(() => ({
}));
// @ts-ignore
const Continue = styled.div(({ theme }) => ({
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
  gap: 40,
  padding: '40px 0 0',

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
      filter: `drop-shadow(0px 0px 5px ${theme.colors.primary.highlight})`,
    },
  },
}));


type InvitationFormProps = {
  onSubmit: (access_token: string) => void;
  unique_id: string;
};

// @ts-ignore
const Title = styled.div(({ theme }) => ({
  fontWeight: 700,
  fontFamily: theme.fonts.primary,
  color: '#cbe6e5',
  fontSize: 36,
}));

// @ts-ignore
const Card = styled.div(({ theme }) => ({
  margin: '20px 0',
  cursor: 'pointer',
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
  background: '#0c464f',
}));

const Table = styled.table(({ theme }) => ({
  borderCollapse: 'collapse',
  width: '100%',
}));
const THead = styled.thead(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  fontSize: 12,
}));
const Th = styled.th(({ theme }) => ({
  padding: 5,
  color: 'rgba(176,78,33,1)',
}));
const Td = styled.td(({ theme }) => ({
  padding: 5,
  color: '#fff',
  fontFamily: theme.fonts.primary,
  fontSize: 18,
  textAlign: 'center',
}));

const isTiktokEnabled = false;

const InvitationForm: React.FC<InvitationFormProps> = ({ onSubmit, unique_id }) => {
  const game = useGameData();

  const [notifications, notify] = useNotifications();
  //const [] = useSearchParams();
  //const [access_token] = useLocalRef<null | string>('settings.access-token', null);
  //const [, invite] = useMutation<Result<Invitation[], any>, Invite>('/invite');
  const momentsGetter = useMemo((): GetMoments | null => {
    if (unique_id) return { unique_id };
    else return null;
  }, [unique_id]);
  const [, momentsQuery] = useQuery<Result<Moment[], any>, GetMoments | null>('/get-moments', momentsGetter);
  const rankingGetter = useMemo((): GetRanking | null => {
    if (unique_id) return { unique_id };
    else return null;
  }, [unique_id]);
  const [, rankingQuery] = useQuery<Result<[Ranking, PlayerRank[]], any>, GetRanking | null>('/get-ranking', rankingGetter);
  const moments = momentsQuery?.Ok ?? [];
  const [mainRanking, headers, ranks] = useMemo(() => {
    if (!rankingQuery?.Ok) return [null, null, null];
    const [main, rankings] = rankingQuery.Ok;

    const headers: string[] = [];
    const ranks: number[][] = [];

    for (const rank of rankings) {
      headers.push(rank.ranking.title);
      if (main.id !== rank.ranking.id) continue;
      for (const [index, score] of rank.scores.entries()) {
        if (!ranks[index]) ranks[index] = [];
        ranks[index].push(score.ranking);
      }
    }
    for (const rank of rankings) {
      for (const [index, score] of rank.scores.entries()) {
        if (!ranks[index]) ranks[index] = [];
        ranks[index].push(score.score);
      }
    }
    return [main, headers, ranks];
  }, [rankingQuery?.Ok]);

  const handleCopyUrl = (url: string) => (event: any) => {
    event.preventDefault();
    event.stopPropagation();
    navigator.clipboard.writeText(url);
    notify("Url copied to your clipboard", 5000);

    game.sendShareEvent();
  };

  const handleCopyMomentUrl = (id: string) => (event: any) => {
    event.preventDefault();
    event.stopPropagation();
    const url = `${window.location.origin}/moment/${id}`;
    navigator.clipboard.writeText(url);
    notify("Url copied to your clipboard", 5000);

    game.sendShareEvent();
  }

  const handleTiktok = (id: string) => () => {
    console.log('tiktok');
  };

  const handleNavigateToMoment = (id: string) => () => {
    window.open(`/moment/${id}`, '_blank');
  };

  return (
    <Main>
      <Sidebar />
      <Content>
        {(ranks && headers && mainRanking) && (
          <>
            <Table>
              <THead>
                <Th>Global Rank</Th>
                {headers.map(rank => (
                  <Th>{rank}</Th>
                ))}
              </THead>
              {ranks.map(rank => (
                <tr>
                  {rank.map(score => (
                    <Td>{score}</Td>
                  ))}
                </tr>
              ))}
            </Table>
            <Hr />
          </>
        )}
      </Content>
      <Stage style={{ textAlign: 'center' }}>
        <div>
          <Title>Moments</Title>
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
              <Row>
                {isTiktokEnabled ? <TiktokShare onClick={handleTiktok(moment.id)} /> : <div />}
                <CopyShare onClick={handleCopyMomentUrl(moment.id)} />
              </Row>
            </Card>
          ))}
        </div>
      </Stage>
      <Footer>
        {notifications.map((notification) => (
          <Alert key={notification.id}>{notification.content}</Alert>
        ))}
        <FancyButton onClick={handleCopyUrl(window.location.href)}>
          Invite Others to Play
        </FancyButton>
        <Continue>
          <GetChaos href="https://discord.gg/6XZ3S6wN" target="_blank" rel="noreferrer">
            <Discord />
            <span>Join our Discord</span>
          </GetChaos>
          <GetChaos href="https://getchaotic.com" target="_blank" rel="noreferrer">
            <PoweredBy />
          </GetChaos>
        </Continue>
      </Footer>
    </Main>
  );
};

export default InvitationForm;
