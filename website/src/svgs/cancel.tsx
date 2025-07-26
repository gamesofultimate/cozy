import styled from '@emotion/styled';
import React from 'react';

const Icon = styled.div(() => ({
  transition: 'color 200ms ease-in-out',
  width: '100%',
  height: '100%',
  cursor: 'pointer',
  transform: 'rotate(-90deg)',

  svg: {
    width: '100%',
    height: '100%',
  },
}));

const Path = styled.path(() => ({
  fill: 'currentcolor',
}));

type CancelProps = {
  onClick?: () => void;
};

const CancelIcon: React.FC<CancelProps> = ({ onClick }) => (
  <Icon onClick={onClick}>
    <svg preserveAspectRatio="xMidYMid meet" viewBox="0 0 21 21" fill="none">
      <Path
        fill-rule="evenodd"
        clip-rule="evenodd"
        d="M10.5008 13.294L15.6793 18.596C16.1988 19.132 16.5368 19.1375 17.0658 18.596L18.1048 17.532C18.6138 17.011 18.6483 16.669 18.1048 16.1125L12.6223 10.5L18.1053 4.88749C18.6188 4.35999 18.6288 4.00399 18.1053 3.46749L17.0663 2.40399C16.5273 1.85199 16.1943 1.87749 15.6798 2.40399L10.5008 7.70599L5.32232 2.40449C4.80783 1.87799 4.47482 1.85249 3.93582 2.40449L2.89682 3.46799C2.37282 4.00449 2.38232 4.36049 2.89682 4.88799L8.37933 10.5L2.89682 16.1125C2.35332 16.669 2.38232 17.011 2.89682 17.532L3.93532 18.596C4.45982 19.1375 4.79782 19.132 5.32182 18.596L10.5008 13.294Z"
        fill="currentColor"
      />
    </svg>
  </Icon>
);

export default CancelIcon;
