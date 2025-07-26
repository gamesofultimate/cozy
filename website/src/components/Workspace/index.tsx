import React from 'react';

import { ThemeProvider } from '@emotion/react';

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
      <s.World focused={focused}>
        <s.Logo focused={focused}>
          {logo?.length > 0 && <s.LogoInner>{logo}</s.LogoInner>}
        </s.Logo>
        <s.Header>
          <s.Left focused={focused}>
            {left?.length > 0 && <s.LeftInner>{left}</s.LeftInner>}
          </s.Left>
          <s.Right focused={focused}>
            <s.Relative>
              {right?.length > 0 && <s.RightInner>{right}</s.RightInner>}
            </s.Relative>
          </s.Right>
        </s.Header>
        {presentation?.length > 0 && (
          <s.PresentationSpace mode={mode}>
            <s.Presentation>{presentation}</s.Presentation>
            {dialog?.length > 0 && <s.Dialogs>{dialog}</s.Dialogs>}
          </s.PresentationSpace>
        )}
        <s.Main focused={focused}>{main}</s.Main>
        <s.Footer>{footer}</s.Footer>
      </s.World>
      {presentation?.length === 0 && dialog?.length > 0 && <s.Dialogs>{dialog}</s.Dialogs>}
      {notification?.length > 0 && <s.Notifications>{notification}</s.Notifications>}
    </ThemeProvider>
  );
};

export default Workspace;
