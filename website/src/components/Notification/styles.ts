import styled from '@emotion/styled';
import { padding, rgba } from 'polished';

type ActivatedProps = {
  activated: boolean;
};
export const Wrapper = styled.div<ActivatedProps>(({ theme, activated }) => [
  {
    position: 'fixed',
    top: 0,
    right: 0,
    bottom: 0,
    left: 0,

    display: 'flex',
    justifyContent: 'center',
    backdropFilter: 'blur(0px)',
    transition: 'background 700ms ease-in-out,backdrop-filter 700ms ease-in-out',
    zIndex: 1000,
    background: rgba(50, 50, 50, 0),
    height: '100%',
  },
  activated && {
    backdropFilter: 'blur(3px)',
    background: rgba(50, 50, 50, 0.25),
  },
]);

export type MainProps = {
  width?: number;
};

export const Main = styled.div<MainProps & ActivatedProps>(({ theme, activated }) => [
  {
    color: '#000',
    overflowY: 'auto',
    transition: 'right 700ms ease-in-out',
  },
  activated && {
    right: 0,
  },
]);

export const Content = styled.div(() => [
  {
    padding: 15,
  },
]);

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
