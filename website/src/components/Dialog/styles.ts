import styled from '@emotion/styled';
import { padding, rgba } from 'polished';

type ActivatedProps = {
  activated: boolean;
};
export const Wrapper = styled.div<ActivatedProps>(({ theme, activated }) => [
  {
    gridArea: 'dialog',
    display: 'flex',
    justifyContent: 'center',
    backdropFilter: 'blur(0px)',
    transition: 'background 700ms ease-in-out,backdrop-filter 700ms ease-in-out',
    zIndex: 1000,
    position: 'relative',
    background: rgba(theme.colors.primary.light, 0.25),
    height: '100%',
  },
  activated && {
    backdropFilter: 'blur(3px)',
    background: rgba(theme.colors.primary.light, 0.25),
  },
]);

export type MainProps = {
  width?: number;
};

export const Main = styled.div<MainProps & ActivatedProps>(({ theme, width = 700, activated }) => [
  {
    width,
    background: '#191919',
    color: theme.colors.basic.black,
    overflowY: 'auto',
    transition: 'right 700ms ease-in-out',
  },
  activated && {
    right: 0,
  },
]);

export const Content = styled.div(() => [{}]);

export const HeaderLine = styled.div(({ theme }) => [
  {
    fontFamily: theme.fonts.primary,
    fontWeight: 700,
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    fontSize: 24,
    color: theme.colors.basic.white,
    ...padding(15),
  },
]);

export const Close = styled.a(({ theme }) => [
  {
    color: theme.colors.basic.white,
    cursor: 'pointer',
    width: 13,
    height: 13,
    display: 'block',
  },
]);
