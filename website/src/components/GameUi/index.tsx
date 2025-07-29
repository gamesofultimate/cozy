import React from 'react';
import { observer } from 'mobx-react';
import styled from '@emotion/styled';

import { UiMode, useGameData } from 'data/game';

// @ts-ignore
const Centered = styled.div(() => ({
  width: 356,
  height: 'calc(100vh - 350px)',
  margin: '0 auto',
  display: 'grid',
  gridTemplateAreas: `
    'storage'
    'action'
    'footer'
  `,
  gridTemplateColumns: 'minmax(0, 1fr)',
  gridTemplateRows: 'minmax(0, 1fr) 100px 100px',
  gap: '10px 10px',
  pointerEvents: 'none',

  position: 'fixed',
  bottom: 20,
  right: 20,
}));


// @ts-ignore
const Action = styled.div(() => ({
  gridArea: 'action',
  overflowY: 'scroll',
  paddingRight: 15,
  textAlign: 'right',
}));

// @ts-ignore
const Storage = styled.div(() => ({
  gridArea: 'storage',
  overflowY: 'scroll',
  paddingRight: 15,
  textAlign: 'right',
}));

// @ts-ignore
const Footer = styled.div(() => ({
  gridArea: 'footer',
  alignSelf: 'end',
  textAlign: 'right',
  paddingRight: 15,
}));

const StoragePadding = styled.div(({ theme }) => ({
  padding: '20px 20px 0px 20px',
}));

const StorageCard = styled.div(({ theme }) => ({
  boxShadow: '3px 3px 16px #a3d9f873',
  backdropFilter: 'blur(7px) brightness(1.3)',
  borderRadius: 40,
  width: '100%',
  height: '100%',
  boxSizing: 'border-box',
}));

const StorageSpace = styled.div(({ theme }) => ({
  display: 'grid',
  gridTemplateColumns: 'repeat(4, 1fr)',
  gridTemplateRows: 'repeat(4, 1fr)',
  gridColumnGap: 10,
  gridRowGap: 10,
  padding: 10,
  width: '100%',
  height: '100%',
  boxSizing: 'border-box',
}));

// @ts-ignore
const StorageItem = styled.div(() => ({
  width: '100%',
  height: '100%',
  borderRadius: 20,
  boxShadow: 'inset 0 0 10px #00000091',
  pointerEvents: 'auto',
  cursor: 'pointer',
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
}));

const Subtitle = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.primary,
  fontSize: 36,
  fontWeight: 700,
  color: '#8B5434',
  lineHeight: '1.0em',
  textShadow: '1px 1px 5px #e1ffe9',
}));

// @ts-ignore
const Body = styled.div(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  color: '#e1ffe9',
  fontSize: 14,
  fontWeight: 700,
  textShadow: '0px 0px 7px #091b0e',
}));

type PauseProps = {
  mode: UiMode;
};

const Pause: React.FC<PauseProps> = ({ mode }) => {
  const game = useGameData();

  const buckets = [...new Array(12)].map(_ => {});
  console.log(buckets);

  return (
    <Centered>
      <Storage>
        <StorageCard>
          <StoragePadding>
            <Subtitle>Backpack</Subtitle>
          </StoragePadding>
          <StorageSpace>
            {buckets.map(item => (
              <StorageItem />
            ))}
          </StorageSpace>
        </StorageCard>
      </Storage>
      <Action>
        <Card>
          <Subtitle>Water the Soil</Subtitle>
          <Body>Current Action</Body>
        </Card>
      </Action>
      <Footer>
        <Card>
          <Subtitle>$ 1,000</Subtitle>
          <Body>Cash</Body>
        </Card>
      </Footer>
    </Centered >
  );
};


export default observer(Pause);
