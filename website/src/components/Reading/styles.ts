import styled from '@emotion/styled';

const CENTERING_OFFSET = 650;
const LOGO_OFFSET = 200;
const TRANSITION_TIMING = '700ms';

export enum WorkspaceMode {
  Working,
  UserInput,
}

type WorkspaceProps = {
  mode: WorkspaceMode;
};

export const World = styled.div(({ theme }) => [
  {
    padding: 0,
    background: 'linear-gradient(0deg, rgba(0,0,0,1) 40%, rgba(12,70,79,1) 70%, rgba(5,40,41,1) 100%)',
    display: 'grid',
    gridTemplateAreas: `
      'logo header menu'
      'content content content'
      'footer footer footer'
    `,
    gridTemplateColumns: '300px minmax(0, 1fr) 300px',
    gridTemplateRows: '160px minmax(0, 1fr) 100px',
    gap: '40px 80px',
  },
]);

// @ts-ignore
export const BackgroundSpace = styled.div(() => ({
  position: 'absolute',
  top: 0,
  right: 0,
  bottom: 0,
  left: 0,
}));

// @ts-ignore
export const LogoSpace = styled.div(() => ({
  gridArea: 'logo',
  alignSelf: 'center',
  zIndex: 10,
}));

export const MenuSpace = styled.div(() => ({
  gridArea: 'menu',
  alignSelf: 'center',
  zIndex: 10,
}));

export const FooterSpace = styled.div(() => ({
  gridArea: 'footer',
}));

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

export enum FocusState {
  Focused,
  Unfocused,
  Relax,
}

type FocusedProps = {
  focused: FocusState;
};

export const PresentationSpace = styled.div<WorkspaceProps>(({ mode }) => [
  {
    gridArea: 'content',
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
  focused === FocusState.Relax && {
    margin: '0 auto',
    maxWidth: 1200,
    textAlign: 'center',
  },
]);

export const Logo = styled.div<FocusedProps>(({ focused }) => [
  {
    transition: `top ${TRANSITION_TIMING} ease-in-out`,
    pointerEvents: 'auto',
  },
]);

export const Left = styled.div<FocusedProps>(({ focused }) => [
  {
    position: 'absolute',
    pointerEvents: 'none',
    transition: `top ${TRANSITION_TIMING} ease-in-out, left ${TRANSITION_TIMING} ease-in-out`,
  },
  (focused === FocusState.Unfocused || focused === FocusState.Relax) && {
    top: 0,
    left: `calc(50% - ${CENTERING_OFFSET + LOGO_OFFSET}px)`,
  },
  focused === FocusState.Focused && {
    top: -900,
    left: -900,
  },
]);

// @ts-ignore
export const LeftInner = styled.div(() => [{}]);

export const Right = styled.div<FocusedProps>(({ focused }) => [
  {
    position: 'absolute',
    pointerEvents: 'none',
    transition: `top ${TRANSITION_TIMING} ease-in-out, right ${TRANSITION_TIMING} ease-in-out`,
  },
  (focused === FocusState.Unfocused || focused === FocusState.Relax) && {
    top: 0,
    right: `calc(50% - ${CENTERING_OFFSET + LOGO_OFFSET}px)`,
  },
  focused === FocusState.Focused && {
    top: -900,
    right: -900,
  },
]);
//
// @ts-ignore
export const RightInner = styled.div(() => [{}]);

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
    background: `linear-gradient(0deg, ${theme.colors.primary.dark} 0%, ${theme.colors.primary.light} 100%)`,
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

export const Footer = styled.div<FocusedProps>(({ focused }) => [
  {
    position: 'absolute',
    pointerEvents: 'none',
    transition: `top ${TRANSITION_TIMING} ease-in-out, right ${TRANSITION_TIMING} ease-in-out`,
  },
  focused === FocusState.Unfocused && {
    top: 0,
    right: `calc(50% - ${CENTERING_OFFSET + LOGO_OFFSET}px)`,
  },
  focused === FocusState.Focused && {
    top: -900,
    right: -900,
  },
]);

// @ts-ignore
export const Dialogs = styled.div(() => ({
  justifyContent: 'center',
}));

// @ts-ignore
export const Notifications = styled.div(() => ({
  justifyContent: 'center',
}));
