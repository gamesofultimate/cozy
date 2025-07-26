import styled from '@emotion/styled';

const TRANSITION_TIMING = '700ms';

export enum WorkspaceMode {
  Working,
  UserInput,
}

type WorkspaceProps = {
  mode: WorkspaceMode;
};

export enum FocusState {
  Focused,
  Unfocused,
  Relax,
}

type FocusedProps = {
  focused: FocusState;
};

export const World = styled.div<FocusedProps>(({ theme, focused }) => [
  {
    position: 'relative',
    padding: 0,
    background: '#A3D9F8',
  },
  focused === FocusState.Focused && {
    height: '100vh',
    overflow: 'hidden',
  },
]);

// @ts-ignore
export const Relative = styled.div(() => ({
  position: 'relative',
  width: '100%',
  height: '100%',
}));

// @ts-ignore
export const Header = styled.div(() => ({
  position: 'absolute',
  width: '100%',
  height: 400,
  overflow: 'hidden',
  pointerEvents: 'none',
}));

export const PresentationSpace = styled.div<WorkspaceProps>(({ mode }) => [
  {
    display: 'grid',
    gridTemplateAreas: "'main dialog'",
    transition: TRANSITION_TIMING,
    width: '100vw',
    height: '100vh',
  },
  mode === WorkspaceMode.Working && {
    gridTemplateColumns: 'minmax(0, 1fr) 0px',
    gridTemplateRows: 'minmax(0, 1fr)',
  },
  mode === WorkspaceMode.UserInput && {
    gridTemplateColumns: 'minmax(0, 1fr) 500px',
    gridTemplateRows: 'minmax(0, 1fr)',
  },
]);

export const Presentation = styled.div(() => [
  {
    gridArea: 'main',
    width: '100%',
    height: '100%',
  },
]);

export const Main = styled.div<FocusedProps>(({ focused }) => [
  {
    position: 'absolute',
    top: 'calc(100vh - 180px)',
    transition: `top ${TRANSITION_TIMING} ease-in-out`,
  },
  focused === FocusState.Focused && {
    top: '100vh',
  },
  focused === FocusState.Relax && {
    paddingTop: 200,
    margin: '0 auto',
    maxWidth: 1200,
  },
]);

export const Logo = styled.div<FocusedProps>(({ focused }) => [
  {
    position: 'absolute',
    top: '10vh',
    transition: `top ${TRANSITION_TIMING} ease-in-out, left ${TRANSITION_TIMING} ease-in-out`,
    width: '100vw',
  },
  (focused === FocusState.Unfocused || focused === FocusState.Relax) && {
    left: 50,
  },
  focused === FocusState.Focused && {
    left: -900,
  },
]);

// @ts-ignore
export const LogoInner = styled.div(() => [
  {
    pointerEvents: 'none',
    margin: '10px 0 10px 0',
    textAlign: 'center',
  },
]);

export const Left = styled.div<FocusedProps>(({ focused }) => [
  {
    position: 'absolute',
    transition: `top ${TRANSITION_TIMING} ease-in-out, left ${TRANSITION_TIMING} ease-in-out`,
  },
  (focused === FocusState.Unfocused || focused === FocusState.Relax) && {
    top: 0,
    left: 0,
  },
  focused === FocusState.Focused && {
    top: -900,
    left: -900,
  },
]);

// @ts-ignore
export const LeftInner = styled.div(() => [
  {
    position: 'absolute',
    pointerEvents: 'none',
    top: 0,
    left: 0,
    margin: '10px 0 10px 0',
  },
]);

export const Right = styled.div<FocusedProps>(({ focused }) => [
  {
    position: 'absolute',
    pointerEvents: 'none',
    transition: `top ${TRANSITION_TIMING} ease-in-out, right ${TRANSITION_TIMING} ease-in-out`,
  },
  (focused === FocusState.Unfocused || focused === FocusState.Relax) && {
    top: 0,
    right: 0,
  },
  focused === FocusState.Focused && {
    top: -900,
    right: -900,
  },
]);
//
// @ts-ignore
export const RightInner = styled.div(() => [
  {
    position: 'absolute',
    pointerEvents: 'none',
    top: 0,
    right: 0,
    margin: '10px 0 10px 0',
  },
]);

export const LeftGreenSphere = styled.div<FocusedProps>(({ theme, focused }) => [
  {
    position: 'absolute',
    pointerEvents: 'none',
    top: -902,
    left: -930,
    width: 1772,
    height: 1251,
    background: `linear-gradient(90deg, ${theme.colors.primary.dark} 0%, ${theme.colors.primary.light} 100%)`,
    maskImage: 'radial-gradient(circle, rgba(0, 0, 0, 1) 0%, rgba(0, 0, 0, 0) 60%)',
    transition: `opacity ${TRANSITION_TIMING} ease-in-out`,
  },
  (focused === FocusState.Unfocused || focused === FocusState.Relax) && {
    opacity: 1,
  },
  focused === FocusState.Focused && {
    opacity: 0,
  },
]);

export const RightGreenSphere = styled.div<FocusedProps>(({ theme, focused }) => [
  {
    position: 'absolute',
    pointerEvents: 'none',
    top: -902,
    right: -930,
    width: 1772,
    height: 1251,
    background: `linear-gradient(90deg, ${theme.colors.primary.dark} 0%, ${theme.colors.primary.light} 100%)`,
    maskImage: 'radial-gradient(circle, rgba(0, 0, 0, 1) 0%, rgba(0, 0, 0, 0) 60%)',
    transition: `opacity ${TRANSITION_TIMING} ease-in-out`,
  },
  (focused === FocusState.Unfocused || focused === FocusState.Relax) && {
    opacity: 1,
  },
  focused === FocusState.Focused && {
    opacity: 0,
  },
]);

export const BlackSphere = styled.div<FocusedProps>(({ focused }) => [
  {
    position: 'absolute',
    pointerEvents: 'none',
    top: -902,
    left: `calc(50% - ${1772 / 2}px)`,
    width: 1772,
    height: 1251,
    background: 'radial-gradient(circle, rgba(0, 0, 0, 1) 0%, rgba(0, 0, 0, 0) 60%)',
    transition: `opacity ${TRANSITION_TIMING} ease-in-out`,
  },
  (focused === FocusState.Unfocused || focused === FocusState.Relax) && {
    opacity: 1,
  },
  focused === FocusState.Focused && {
    opacity: 0,
  },
]);

export const Footer = styled.div(() => [{}]);

// @ts-ignore
export const Dialogs = styled.div(() => ({
  justifyContent: 'center',
}));

// @ts-ignore
export const Notifications = styled.div(() => ({
  justifyContent: 'center',
}));
