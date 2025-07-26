import React, { useEffect, useMemo, useRef } from 'react';
import { Helmet } from "react-helmet";

//import { useNavigate, useSearchParams } from 'react-router-dom';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { observer } from 'mobx-react';
//import plataform from 'platform-detect';

import * as uuid from 'uuid';

import { Result, Moment, Ranking, PlayerRank } from '@ultimate-games/canvas';
import Dialog, { Content as DialogContent } from 'components/Dialog';
import Notification, { Content as NotificationContent } from 'components/Notification';
import Workspace, { Footer, Logo, Main, Presentation, Left, Right } from 'components/Workspace';
import InfoBlock from 'components/InfoBlock';

import Logo1x from 'images/logo@1x.png';
import Logo2x from 'images/logo@2x.png';

import Pirate1x from 'images/pirate-face@1x.png';
import Pirate2x from 'images/pirate-face@2x.png';

import Story1x from 'images/story@1x.png';
import Story2x from 'images/story@2x.png';

import Moon1x from 'images/moon@1x.png';
import Moon2x from 'images/moon@2x.png';

import Screenshot from 'images/screenshot.png';

import Survival1x from 'images/survival@1x.png';
import Survival2x from 'images/survival@2x.png';

import FancyButton from 'components/FancyButton';
import LoggedOutMenu from 'components/LoggedOutMenu';
import LoggedInMenu from 'components/LoggedInMenu';
import Subtitle from 'components/Subtitle';
import SignupForm from 'components/SignupForm';
import { Discord } from 'svgs/SocialMedia';
import { relative } from 'utils/datetime';

import { NetworkedCanvas, RecordingType, Auth, GameSession, useGpuInfo } from '@ultimate-games/canvas';
import { Access, GetGlobalMoments, GetGlobalRanking, GetOrCreateSession } from 'types';

import styled from '@emotion/styled';
import Box from 'components/Box';
import ThinBox from 'components/ThinBox';
import Image from 'components/Image';

import Chaotic from 'svgs/Chaotic';

//import { useVisitorData } from '@fingerprintjs/fingerprintjs-pro-react';
import { useLocalState, useLocalRef, useCache } from 'hooks/useCacheState';
//import { useLocalState, useCache } from 'hooks/useCacheState';
import { useQuery, useTrigger } from 'hooks/useBackend';
import { gameBus, useGameData } from 'data/game';
import InvitationForm from 'components/InvitationForm';
import Wishlist from 'components/Wishlist';
import Pause from 'components/Pause';
import OutOfCapacity from 'components/OutOfCapacity';

import UnsupportedGpu from 'components/UnsupportedGpu';

// @ts-ignore
const Video = styled.video(() => ({
  width: '100%',
  height: 'auto',
  boxShadow: '0 4px 150px #303030',
}));

// @ts-ignore
const MoreContent = styled.div(() => [
  {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    transition: 'margin-top 700ms ease-in-out',
    backdropFilter: 'blur(10px)',

    borderImage: 'linear-gradient(to right, rgba(255,255,255,0.2) 0%, rgba(153,153,153,0) 100%) 1',
    borderWidth: '1px',
    borderStyle: 'solid',
    padding: 1,
    borderRight: 0,
    borderBottom: 0,
    borderLeft: 0,
  },
]);

// @ts-ignore
const Moments = styled.div(() => [
  {
    display: 'flex',
    justifyContent: 'center',
  },
]);

// @ts-ignore
const Centered = styled.div(() => [
  {
    display: 'flex',
    justifyContent: 'center',
    transition: 'margin-top 700ms ease-in-out',
  },
]);

// @ts-ignore
const Description = styled.div(() => [
  {
    width: 700,
    margin: '0 auto',
    padding: '40px 0',
  },
]);

// @ts-ignore
const Title = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.primary,
  color: '#fff',
  textTransform: 'uppercase',
  letterSpacing: '.5em',
  fontSize: 64,
  textShadow: '0 0 19px #fff',
}));

// @ts-ignore
const Subheader = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  color: '#fff',
  fontSize: 22,
  background: '#ca551e',
  // @ts-ignore
  // eslint-disable-next-line no-dupe-keys
  background: 'linear-gradient(to right, #CA551E 0%, #FFB235 100%)',
  backgroundClip: 'text',
  textFillColor: 'transparent',
}));

// @ts-ignore
const ChaoticLogo = styled.a(() => ({
  display: 'block',
  color: '#fff',
  padding: 20,
  margin: '0 auto',
  pointerEvents: 'auto',
  boxSizing: 'border-box',
  transition: 'color 300ms ease-in-out',
  cursor: 'pointer',
  width: 250,

  ':hover': {
    color: '#fd4a14',
  },
}));

// @ts-ignore
const Card = styled.div(({ theme }) => ({
  margin: '20px 0',
  cursor: 'pointer',
  maxWidth: '20%',
  padding: '0 10px',
  position: 'relative',
}));

const Label = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  color: '#545454',
  fontSize: 12,
  position: 'absolute',
  bottom: 17,
  left: 17,
}));

const Text = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  color: '#aaa',
  fontSize: 16,
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

type MemoProps = {
  config: any;
};
// @ts-ignore
const Memo: React.FC<MemoProps> = React.memo(({ config }) => {
  return <NetworkedCanvas config={config} />;
  //return <div style={{ background: '#000', width: '100%', height: '100%' }} />;
});

// @ts-ignore
const Continue = styled.div(({ theme }) => ({
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
  color: '#fff',
  gap: 40,
  padding: '20px',
  a: {
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

const MainPage: React.FC = () => {
  //const [] = useSearchParams();
  const game = useGameData();
  const [searchParams] = useSearchParams();
  //const isComputer = plataform.windows || plataform.macos || plataform.linux;

  const [access_token, setAccessToken] = useLocalState<null | string>('settings.access-token', null);
  const [uniqueId] = useLocalRef<string>('settings.uniqueId', uuid.v4());
  const connectionId = useRef<string>(uuid.v4());
  const navigate = useNavigate();
  const query = useMemo(() => {
    if (access_token) return { access_token };
    else return null;
  }, [access_token]);
  const momentsGetter = useMemo((): GetGlobalMoments | null => {
    return { };
  }, []);
  const [, momentsQuery] = useQuery<Result<Moment[], any>, GetGlobalMoments | null>('/get-global-moments', momentsGetter);
  const moments = momentsQuery?.Ok ?? [];
  const rankingGetter = useMemo((): GetGlobalRanking | null => {
    return { };
  }, []);
  const [, rankingQuery] = useQuery<Result<[Ranking, PlayerRank[]], any>, GetGlobalRanking | null>('/get-global-ranking', rankingGetter);
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

  const [loadingUser, userQuery] = useQuery<Result<Auth, any>, Access | null>('/user', query);
  const [, sessionQuery, triggerSession] = useTrigger<Result<GameSession, any>, GetOrCreateSession | null>(
    '/get-session',
    //isComputer,
  );

  const branch = searchParams.get('branch');

  const developer = searchParams.get('developer');

  /*
  // NOTE: We can request that this data be returned to us encrypted, so that we can decrypt it
  // in the backend for security. I'll do that later.
  const { data: uniqueId } = useVisitorData({ extendedResult: true }, { immediate: true });
  */

  useEffect(() => {
    if (!connectionId) return;

    triggerSession({
      //unique_key: uniqueId.current,
      unique_key: connectionId.current,
      branch: branch ?? 'main',
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [connectionId, triggerSession]);

  useEffect(() => {
    if (!developer) return;

    setTimeout(() => {
      game.setDeveloperMode(developer);
    }, 1000);
  }, [game, developer]);

  useEffect(() => {
    console.log('access_token is changed', access_token);
  }, [access_token]);

  useEffect(() => {
    console.log('sessionQuery is changed', sessionQuery?.Ok);
  }, [sessionQuery?.Ok]);

  useEffect(() => {
    if (sessionQuery?.Err) {
      // Session is invalid, trigger the OutOfCapacity dialog
      game.setOutOfCapacity();
    }
  }, [sessionQuery?.Err, game]);

  const config = useCache(() => {
    if (!sessionQuery?.Ok) return null;
    if (!uniqueId) return null;

    const session = sessionQuery.Ok.session;
    const version = sessionQuery.Ok.version;

    const recordingConfig: RecordingType = { Id: [session.recording_url, session.game_id] };
    const cacheBurst = uuid.v4();


    return {
      accessToken: access_token,
      connectionId: connectionId.current,
      uniqueId,
      assetLocation: `${version.asset_storage}/resources/`,
      clientJs: `${version.client_storage}/pkg/${version.executable}.js?${cacheBurst}`,
      sessionId: session.id,
      bus: gameBus,
      storageLocation: `${version.asset_storage}/`,
      tcpUrl: `${session.tcp_url ?? ''}`,
      udpUrl: `${session.udp_url ?? ''}`,
      recordingConfig,
    };
    /*
    return null;
     */
  }, [access_token, uniqueId, sessionQuery?.Ok]);

  const handleSubmit = (access_token: string) => {
    game.finishSignup();
    setAccessToken(access_token);
  };

  const handleInvite = () => {
    game.closeInvitationDialog();
  };

  const handleLogout = () => {
    setAccessToken(null);
  };

  const handleNavigateToMoment = (id: string) => () => {
    window.open(`/moment/${id}`, '_blank');
  };

  const gpuInfo = useGpuInfo();

  if (!gpuInfo) {
    return <div></div>;
  }

  console.log('Gpu info', gpuInfo);

  /*
  if (plataform.macos) {
    gpuInfo.tier -= 1;
  }
  */

  if (gpuInfo.tier >= 2) {
    return (
      <>
        <Helmet>
          <meta name="description" content="Play now in your browser. Moonfallen is a fast-paced space opera from Chaotic Games. Fight enemies and escape with friends. Will you survive?" />
          <title>Moonfallen | Play Now</title>

          {moments.length > 0 && (
            <>
              <meta property="og:video" content={`${window.location.origin}${moments[0].video_url}`} />
              <meta property="og:video:alt" content="The most popular moment of Moonfallen so far" />
              <meta property="og:video:width" content="1920" />
              <meta property="og:video:height" content="1080" />
            </>
          )}
          
          <meta property="og:image" content={`${window.location.origin}${Screenshot}`} />
          <meta property="og:image:alt" content="An image with dark background featuring a large panther on the left. The title — Light and dark mode in just 14 lines of CSS — is in white text at the centre, and below are icons representing the topics of the shared page." />
          <meta property="og:image:width" content="3570" />
          <meta property="og:image:height" content="2048" />

          <meta name="twitter:label1" content="Created by" />
          <meta name="twitter:data1" content="Chaotic Games" />

          <meta property="og:type" content="game" />

          <meta name="twitter:card" content="summary_large_image" />
          <meta name="twitter:site" content="@moonfallengame" />
          <meta name="twitter:creator" content="@moonfallengame" />
        </Helmet>
        <Workspace
          focused={game.focused}
          mode={game.workspaceMode}
        >
          <Logo>
            <Title>Moonfallen</Title>
            <Subheader>You must escape before it's too late.</Subheader>
          </Logo>
          <Left>
            <ChaoticLogo href="https://www.getchaotic.com/" target="_blank">
              <Chaotic />
            </ChaoticLogo>
          </Left>
          <Right>
            {!loadingUser && userQuery && (
              <>
                {userQuery.Ok && <LoggedInMenu username={userQuery.Ok.username} onLogout={handleLogout} />}
                {!userQuery.Ok && <LoggedOutMenu onSignup={() => game.toggleSignup()} />}
              </>
            )}
          </Right>
          <Presentation>
            {config ? (
              <Memo key="game" config={config} />
            ) : (
              <div style={{ background: '#000', width: '100%', height: '100%' }} />
            )}
          </Presentation>
          <Main>
            <MoreContent>
              {(ranks && headers && mainRanking) ? (
                <>
                  <Table>
                    <THead>
                      <Th>#</Th>
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
                </>
              ) : (
                <div />
              )}
              <Moments>
                {moments.slice(0, 3).map((moment) => (
                  <Card key={moment.id} onClick={handleNavigateToMoment(moment.id)}>
                    <Video autoPlay loop muted crossOrigin='anonymous'>
                      <source src={moment.video_url} />
                      Download the <a href={moment.video_url}>video demo</a>.
                    </Video>
                    <Label>{relative(new Date(moment.created_at))}</Label>
                  </Card>
                ))}
              </Moments>
              <div style={{ padding: 10 }}>
                <FancyButton>
                  Play
                </FancyButton>
              </div>
            </MoreContent>
          </Main>
          <Footer>
            <Description>
              <Subtitle>Moonfallen</Subtitle>
              <Text>
                Hundreds of years ago, bio-engineered soldiers were developed in secret as a deterrent to any and all opposition to the government of Earth. Over time, whispers of these soldiers developed a mythos around their being and their incredible capabilities. The year is 3077, and Martian Republic and Earthen Alliance tensions are at an all-time high. Reports received from all over the world indicate the moon is under attack. As Earth scrambles to respond, the Martians take strategic facilities around the Moon in hopes of discovering these soldiers and destroying them.
              </Text>
            </Description>

            <div style={{ display: 'flex', width: '100%', justifyContent: 'space-evenly' }}>
              <div style={{ padding: 15, textAlign: 'center' }}>
                <Subtitle>Created and Developed by <br />Chaotic Games</Subtitle>
                <ChaoticLogo href="https://www.getchaotic.com/" target="_blank">
                  <Chaotic />
                </ChaoticLogo>
              </div>
            </div>
            <div>
              <Image source={Moon1x} retina={Moon2x} alt="moon" />
            </div>
            <Continue>
              <a href="https://discord.gg/ag46Jufg7F" target="_blank" rel="noreferrer">
                <Discord />
                <span>Join our Discord</span>
              </a>
            </Continue>
          </Footer>
          {game.pauseDialog && (
            <Notification>
              <NotificationContent>
                <ThinBox>
                  <Pause unique_id={uniqueId} onClose={() => game.closePauseDialog()} />
                </ThinBox>
              </NotificationContent>
            </Notification>
          )}
          {
            game.isSigningUp && (
              <Dialog onClose={() => game.finishSignup()}>
                <DialogContent>
                  <SignupForm onSubmit={handleSubmit} />
                </DialogContent>
              </Dialog>
            )
          }
          {
            game.invitationDialog && (
              <Dialog>
                <DialogContent>
                  <InvitationForm unique_id={uniqueId} onSubmit={handleInvite} />
                </DialogContent>
              </Dialog>
            )
          }
          {
            game.wishlistDialog && (
              <Notification>
                <NotificationContent>
                  <ThinBox>
                    <Wishlist
                      kills={game.kills}
                      deaths={game.deaths}
                      time={game.time}
                      unique_id={uniqueId}
                      onClose={() => game.closeWishlistDialog()}
                    />
                  </ThinBox>
                </NotificationContent>
              </Notification>
            )
          }
          {game.outOfCapacityDialog && (
            <Notification>
              <NotificationContent>
                <ThinBox>
                  <OutOfCapacity onClose={() => game.closeOutOfCapacityDialog()} />
                </ThinBox>
              </NotificationContent>
            </Notification>
          )}
        </Workspace >
      </>
    );
  }

  return (
    <Workspace focused={game.focused} mode={game.workspaceMode} >
      <Logo>
        <Image source={Logo1x} retina={Logo2x} alt="Mark of the Deep logo" onClick={() => navigate('/')} />
      </Logo>
      <Right>
        {!loadingUser && userQuery && (
          <>
            {userQuery.Ok && <LoggedInMenu username={userQuery.Ok.username} onLogout={handleLogout} />}
            {!userQuery.Ok && <LoggedOutMenu onSignup={() => game.toggleSignup()} />}
          </>
        )}
      </Right>
      <Main>
        <Notification>
          <NotificationContent>
            <ThinBox>
              <UnsupportedGpu />
            </ThinBox>
          </NotificationContent>
        </Notification>
        <Centered>
          <Box>
            <InfoBlock
              title="Character"
              image={{ main: Pirate1x, retina: Pirate2x, alt: 'Character' }}
              description="An epic pirate-themed adventure in thrilling mix of Metroidvania and Souls-Like elements."
            />
            <InfoBlock
              title="Story"
              image={{ main: Story1x, retina: Story2x, alt: 'Story' }}
              description="Explore biomes and fight abyssal monsters to find your lost crew in a cursed world."
            />
            <InfoBlock
              title="Survival"
              image={{ main: Survival1x, retina: Survival2x, alt: 'Survival' }}
              description="Explore biomes and fight abyssal monsters to find your lost crew in a cursed world."
            />
          </Box>
        </Centered>
      </Main>
      <Footer>
        <div style={{ display: 'flex', width: '100%', justifyContent: 'space-evenly' }}>
          <div style={{ padding: 15, textAlign: 'center' }}>
            <Subtitle>Powered by <br />Chaotic Games</Subtitle>
            <ChaoticLogo href="https://www.getchaotic.com/" target="_blank">
              <Chaotic />
            </ChaoticLogo>
          </div>
        </div>
      </Footer>
      {
        game.isSigningUp && (
          <Dialog onClose={() => game.finishSignup()}>
            <DialogContent>
              <SignupForm onSubmit={handleSubmit} />
            </DialogContent>
          </Dialog>
        )
      }
    </Workspace >
  );
};

export default observer(MainPage);
