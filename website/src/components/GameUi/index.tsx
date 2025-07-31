import React from 'react';
import { observer } from 'mobx-react';
import styled from '@emotion/styled';

import { UiMode, useGameData } from 'data/game';

import PumpkinCrop from 'images/crops/pumpkin-crops.png';
import PumpkinSeeds from 'images/crops/pumpkin-seeds.png';
import ProgressBar from 'components/ProgressBar';

type CenteredProps = {
  mode: UiMode;
};

// @ts-ignore
const Centered = styled.div<CenteredProps>(({ mode }) => [
    {
    //width: '100%',
    height: 250,
    // height: 'calc(100vh - 350px)',
    margin: '0 auto',
    display: 'grid',
    gridTemplateAreas: `
      'character storage action'
      'character storage footer'
    `,
    gridTemplateColumns: '300px minmax(0, 1fr) 300px',
    gridTemplateRows: 'minmax(0, 1fr) 100px',
    gap: '10px 10px',
    pointerEvents: 'none',

    position: 'fixed',
    bottom: 20,
    right: 20,
    left: 20,
    transition: 'opacity 300ms ease-in-out',
  },
  mode === UiMode.Hidden && {
    opacity: 0,
  },
  mode !== UiMode.Hidden && {
    opacity: 1,
  },
]);

// @ts-ignore
const Character = styled.div(() => ({
  gridArea: 'character',
  overflowY: 'scroll',
  paddingRight: 15,
  textAlign: 'left',
}));

// @ts-ignore
const Storage = styled.div(() => ({
  gridArea: 'storage',
  overflowY: 'scroll',
  paddingRight: 15,
  textAlign: 'right',
}));

// @ts-ignore
const Action = styled.div(() => ({
  gridArea: 'action',
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
  gridTemplateColumns: 'repeat(5, minmax(0, 1fr))',
  gridTemplateRows: 'repeat(2, minmax(0, 1fr))',
  gridColumnGap: 10,
  gridRowGap: 10,
  padding: 10,
  width: '100%',
  height: '100%',
  boxSizing: 'border-box',
}));

// @ts-ignore
const StorageBucket = styled.div(() => ({
  width: '100%',
  height: '100%',
  borderRadius: 20,
  boxShadow: 'inset 0 0 10px #00000091',
  pointerEvents: 'auto',
  cursor: 'pointer',
  position: 'relative',
}));

// @ts-ignore
const StorageItem = styled.img(() => ({
  width: '100%',
  height: '100%',
  borderRadius: 20,
  boxShadow: 'inset 0 0 10px #00000091',
  pointerEvents: 'auto',
  cursor: 'pointer',
  position: 'relative',
  objectFit: 'cover',
}));

// @ts-ignore
const QuantityTag = styled.div(({ theme }) => ({
  width: 30,
  height: 30,
  fontFamily: theme.fonts.secondary,
  fontSize: 14,
  position: 'absolute',
  bottom: -7,
  right: -5,
  lineHeight: '28px',
  background: '#B64040',
  borderRadius: 999,
  color: '#fff',
  textAlign: 'center',
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

type GameUiProps = {
};

const GameUi: React.FC<GameUiProps> = () => {
  const game = useGameData();

  return (
    <Centered mode={game.ui.mode}>
      <Character>
        <Card>
          <Subtitle>Name</Subtitle>
          <div>
            <Body>Rest</Body>
            <ProgressBar percent={game.ui.rest} />
          </div>
          <div>
            <Body>Hunger</Body>
            <ProgressBar percent={game.ui.hunger} />
          </div>
          <div>
            <Body>Social</Body>
            <ProgressBar percent={game.ui.social} />
          </div>
        </Card>
      </Character>
      <Storage>
        <StorageCard>
          <StorageSpace>
            {game.ui.inventory.map(inventory => {
              let item = null;
              if (!inventory) {
                return <StorageBucket />;
              } else if (inventory?.item === "Nothing") {
              } else if ("Crop" in inventory.item && inventory?.item?.Crop === "Pumpkin") {
                item = PumpkinCrop;
              } else if ("Seed" in inventory.item && inventory?.item?.Seed === "Pumpkin") {
                item = PumpkinSeeds;
              }
            
              if (!item) {
                return <StorageBucket />;
              }

              if (inventory.quantity === "Empty") {
                return <StorageBucket />;
              } else if (inventory.quantity === "Infinite") {
                return (
                  <StorageBucket>
                    <StorageItem src={item} />
                  </StorageBucket>
                )
              } else if ("Finite" in inventory.quantity) {
                return (
                  <StorageBucket>
                    <StorageItem src={item} />
                    <QuantityTag>{inventory.quantity.Finite}</QuantityTag>
                  </StorageBucket>
                )
              } else {
                return null
              }
            })}
          </StorageSpace>
        </StorageCard>
      </Storage>
      <Action>
        <Card>
          <Subtitle>{game.ui.current_action}</Subtitle>
          <Body>Current Action</Body>
        </Card>
      </Action>
      <Footer>
        <Card>
          <Subtitle>$ {game.ui.cash}</Subtitle>
          <Body>Cash</Body>
        </Card>
      </Footer>
    </Centered >
  );
};


export default observer(GameUi);
