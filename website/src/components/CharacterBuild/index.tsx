import React, {useLayoutEffect, useRef} from 'react';
import { observer } from 'mobx-react';
import styled from '@emotion/styled';
import { Formik, Form } from 'formik';

import Text from 'components/Text';
import Button, { ButtonKind } from 'components/Button';
import ButtonSelect from 'components/ButtonSelect';

import Blondie from 'images/character-select/blondie.png';
import Barry from 'images/character-select/barry.png';

import { useGameData } from 'data/game';

import type {
  GameState,
  // @ts-ignore
} from 'types/ultimate';

type CenteredProps = {
  mode: GameState;
};

function isInViewport(el: Element): boolean {
  const rect = el.getBoundingClientRect();
  return (
    rect.top >= 0 &&
    rect.left >= 0 &&
    rect.bottom <= (window.innerHeight || document.documentElement.clientHeight) &&
    rect.right <= (window.innerWidth || document.documentElement.clientWidth)
  );
}

// @ts-ignore
const Centered = styled.div<CenteredProps>(({ mode }) => [
    {
    //width: '100%',
    // height: 'calc(100vh - 350px)',
    margin: '0 auto',
    display: 'grid',
    gridTemplateAreas: `
      'character main'
    `,
    gridTemplateColumns: '60% minmax(0, 1fr)',
    gridTemplateRows: 'minmax(0, 1fr)',
    gap: '10px 10px',

    position: 'fixed',
    top: 20,
    right: 20,
    bottom: 20,
    left: 20,
    transition: 'opacity 300ms ease-in-out',
  },
  mode !== "Signup" && {
    opacity: 0,
    pointerEvents: 'none',
    //opacity: 1,
  },
  mode === "Signup" && {
    pointerEvents: 'auto',
    opacity: 1,
  },
]);

// @ts-ignore
const Main = styled.div(() => ({
  gridArea: 'main',
  overflowY: 'scroll',
  paddingRight: 15,
  textAlign: 'left',
}));

// @ts-ignore
const Page = styled.div(() => ({
  padding: '42vh 0 50vh',
}));

// @ts-ignore
const Card = styled.div(({ theme }) => ({
  boxShadow: '3px 3px 16px #a3d9f873',
  backdropFilter: 'blur(7px) brightness(1.3)',
  padding: '20px 40px',
  borderRadius: 40,
  width: '100%',
  height: '100%',
  boxSizing: 'border-box',
  pointerEvents: 'auto',
  overflowY: 'scroll',
}));

// @ts-ignore
const Subtitle = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.primary,
  fontSize: 36,
  fontWeight: 700,
  color: '#B64040',
  lineHeight: '1.0em',
  textShadow: '1px 1px 5px #e1ffe9',
}));

// @ts-ignore
const Body = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  color: '#e1ffe9',
  fontSize: 24,
  fontWeight: 700,
  lineHeight: '1.4em',
  textShadow: '0px 0px 7px #091b0e',
}));

const Label = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  color: '#e1ffe9',
  fontSize: 12,
  fontWeight: 700,
  lineHeight: '1.4em',
  textShadow: '0px 0px 7px #091b0e',
}));

type CharacterBuilder = {
  nickname: string;
  username: string;
  password: string;
  confirm_password: string;
  email: string;
  meadow_name: string;
  character_select: string;
};

const initialValues:CharacterBuilder = {
  nickname: '',
  username: '',
  password: '',
  confirm_password: '',
  email: '',
  meadow_name: '',
  character_select: '',
};


type CharacterBuildProps = {
};

const anchors = [
  'welcome',
  'account',
  'meadow',
  'character',
];

const next: { [key: string]: string | null } = {
  'welcome': 'account',
  'account': 'meadow',
  'meadow': 'character',
  'character': null,
};

const CharacterBuild: React.FC<CharacterBuildProps> = () => {
  const game = useGameData();
  const latest = useRef('welcome'); 

  useLayoutEffect(() => {
    const container = document.getElementById('scroller');

    const elements = anchors.map((anchor):[string, Element | null] => [anchor, document.getElementById(anchor)]);

    container?.addEventListener('scroll', () => {
      for (const [anchor, element] of elements) {
        if (element && isInViewport(element)) latest.current = anchor;
      }
    });
  }, []);

  const handleSubmit = async (values: CharacterBuilder) => {
    const current = latest.current;
    const nextPage = next[current];

    if (!nextPage) {
      // last page
      return;
    }

    const el = document.getElementById(nextPage);
    if (el) {
      el.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }

    //const response = await signup(values);
    /*
    if (response?.Ok) {
      setAccessToken(response.Ok.access_token);
      onSubmit(response.Ok.access_token);
    } else {
      console.error(response?.Err);
      notify('Could not sign you up. Try again later.', 5000);
    }
     */
  };

  const validate = async (values: CharacterBuilder) => {
    console.log(values);
    //const response = await signup(values);

    /*
    if (response?.Ok) {
      setAccessToken(response.Ok.access_token);
      onSubmit(response.Ok.access_token);
    } else {
      console.error(response?.Err);
      notify('Could not sign you up. Try again later.', 5000);
    }
     */
  };


  return (
    <Centered mode={game.machine.state}>
      <Main>
        <Card id="scroller">
          <Formik<CharacterBuilder>
            initialValues={initialValues}
            onSubmit={handleSubmit}
            validate={validate}
          >
            {() => (
              <Form>
                <div id="welcome" />
                <Page>
                  <Body>Welcome to</Body>
                  <Subtitle>Fireflies Meadow</Subtitle>
                  <Body>What should we call you? 😊</Body>
                  <Text name="nickname" label="Your nick name" />
                  <Button kind={ButtonKind.Large}>
                    Press Enter to Continue
                  </Button>
                </Page>
                <div id="account" />
                <Page>
                  <Subtitle>Fireflies Meadow</Subtitle>
                  <Body>
                    Lovely name! 🌿  
                    Let's create your account so you can save your meadow and come back anytime.
                  </Body>
                  <Text name="username" label="Username" />
                  <Text name="email" label="E-mail" />
                  <Text name="password" label="Password" type="password" />
                  <Text name="confirm_password" label="Confirm Password" type="password" />
                  <Button kind={ButtonKind.Large}>
                    Press Enter to Continue
                  </Button>
                </Page>
                <div id="meadow" />
                <Page>
                  <Subtitle>Your Meadow</Subtitle>
                  <Body>
                    Every meadow has a name. 🌼  
                    What would you like to call yours?
                  </Body>
                  <Text name="meadow_name" label="Meadow name" />
                  <Label>(Don't worry, you can change it later!)</Label>
                  <Button kind={ButtonKind.Large}>
                    Press Enter to Continue
                  </Button>
                </Page>
                <div id="character" />
                <Page>
                  <Subtitle>Choose Your Character</Subtitle>
                  <Body>
                    Who would you like to explore the meadow as?
                  </Body>

                  <ButtonSelect name='character_select' options={[
                      {
                        source: Blondie,
                        description: 'Brave and gentle, ready for adventure.',
                        value: 'blondie',
                      },
                      {
                        source: Barry,
                        description: 'Curious and kind, with a heart full of wonder.',
                        value: 'barry',
                      },
                    ]}
                  />
                  <Button submit kind={ButtonKind.Large}>
                    Press Enter to Continue
                  </Button>
                  {/*!loading*/ true && (
                    null
                  )}
                </Page>
              </Form>
            )}
          </Formik>
        </Card>
      </Main>
    </Centered >
  );
};


export default observer(CharacterBuild);
