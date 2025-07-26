import React from 'react';

import { ThemeProvider } from '@emotion/react';

import Background from 'svgs/Background';

import { groupBy } from 'lodash';
import { MainTheme } from 'utils/theme';

import * as s from './styles';

export { FocusState, WorkspaceMode } from './styles';

export const Logo: React.FC<React.PropsWithChildren> = ({ children }) => <>{children}</>;
Logo.displayName = 'logo';

export const Presentation: React.FC<React.PropsWithChildren> = ({ children }) => <>{children}</>;
Presentation.displayName = 'presentation';

export const Main: React.FC<React.PropsWithChildren> = ({ children }) => <>{children}</>;
Main.displayName = 'main';

export const Left: React.FC<React.PropsWithChildren> = ({ children }) => <>{children}</>;
Left.displayName = 'left';

export const Right: React.FC<React.PropsWithChildren> = ({ children }) => <>{children}</>;
Right.displayName = 'right';

export const Footer: React.FC<React.PropsWithChildren> = ({ children }) => <>{children}</>;
Footer.displayName = 'footer';

type WorkspaceProps = React.PropsWithChildren & {
  focused?: s.FocusState;
  mode?: s.WorkspaceMode;
};

const Workspace: React.FC<WorkspaceProps> = ({
  children,
  focused = s.FocusState.Relax,
  mode = s.WorkspaceMode.Working,
}) => {
  const nodes = React.Children.toArray(children);

  const {
    logo,
    main,
    presentation,
    left,
    right,
    footer,
    dialog,
    notification,
    // @ts-ignore
  } = groupBy(nodes, (node) => node.type.displayName);

  return (
    <ThemeProvider theme={MainTheme}>
      <s.World>
        <s.BackgroundSpace>
          <Background />
        </s.BackgroundSpace>
        <s.LogoSpace>
          <s.Logo focused={focused}>{logo}</s.Logo>
        </s.LogoSpace>
        <s.MenuSpace>
          {left?.length > 0 && <s.LeftInner>{left}</s.LeftInner>}
          {right?.length > 0 && <s.RightInner>{right}</s.RightInner>}
        </s.MenuSpace>
        <s.PresentationSpace mode={mode}>
          {presentation}
          <s.Main focused={focused}>{main}</s.Main>
        </s.PresentationSpace>
        <s.FooterSpace>{footer}</s.FooterSpace>
      </s.World>
      {presentation?.length === 0 && dialog?.length > 0 && <s.Dialogs>{dialog}</s.Dialogs>}
      {notification?.length > 0 && <s.Notifications>{notification}</s.Notifications>}
    </ThemeProvider>
  );
};

export default Workspace;
