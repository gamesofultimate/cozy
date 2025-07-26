import React from 'react';
import styled from '@emotion/styled';
import { EventProps } from 'utils/events';

export enum EyebrowMode {
  Primary,
  Secondary,
  Special,
}
type EyebrowProps = ModeProps & {
  children: string | React.ReactNode;
};

type ModeProps = {
  mode?: EyebrowMode;
};
const Copy = styled.div<ModeProps & EventProps>(({ theme, mode = EyebrowMode.Primary, onClick }) => [
  {
    textTransform: 'uppercase',
    lineHeight: '1.0em',
    fontWeight: 400,
    fontFamily: theme.fonts.secondary,
    color: theme.colors.basic.white,
  },
  mode === EyebrowMode.Primary && {
    fontSize: 22,
    color: theme.colors.basic.white,
  },
  mode === EyebrowMode.Secondary && {
    fontSize: 10,
    color: theme.colors.basic.gray4,
  },
  onClick && {
    cursor: 'pointer',
  },
]);

const Eyebrow: React.FC<EyebrowProps & EventProps> = ({ children, mode = EyebrowMode.Primary, onClick }) => {
  return (
    <Copy mode={mode} onClick={onClick}>
      {children}
    </Copy>
  );
};

export default Eyebrow;
