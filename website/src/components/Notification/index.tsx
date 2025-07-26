import React, { useState, useEffect } from 'react';

import { ThemeProvider } from '@emotion/react';

import useMousetrap from 'hooks/useMousetrap';
import Cancel from 'svgs/cancel';
import { MainTheme } from 'utils/theme';

import * as s from './styles';

export const Content: React.FC<React.PropsWithChildren> = ({ children }) => <>{children}</>;
Content.displayName = 'content';
export const Header: React.FC<React.PropsWithChildren> = ({ children }) => <>{children}</>;
Header.displayName = 'header';

type NotificationProps = {
  onClose?: () => void;
  width?: number;
  children: React.ReactNode;
};

const Notification: React.FC<NotificationProps> = ({ children, onClose }) => {
  const handleClose = (event?: React.MouseEvent<HTMLElement>) => {
    event?.stopPropagation();
    event?.preventDefault();

    if (onClose) {
      setActivated(false);

      setTimeout(() => {
        onClose();
      }, 700);
    }
  };

  const handleStop = (event?: React.MouseEvent<HTMLElement>) => {
    event?.stopPropagation();
  };

  const [activated, setActivated] = useState(false);
  useMousetrap('escape', () => {
    handleClose();
  });

  useEffect(() => {
    setActivated(true);
  }, [setActivated]);

  const nodes = React.Children.toArray(children);

  // @ts-ignore
  const content = nodes.filter((child) => child.type.displayName === 'content');
  // @ts-ignore
  const header = nodes.filter((child) => child.type.displayName === 'header');

  return (
    <ThemeProvider theme={MainTheme}>
      <s.Wrapper activated={activated} onClick={handleClose}>
        <s.Main activated={activated} onClick={handleStop}>
          <s.HeaderLine>
            {header}
            {onClose && (
              <s.Close onClick={handleClose}>
                <Cancel />
              </s.Close>
            )}
          </s.HeaderLine>
          <s.Content>{content}</s.Content>
        </s.Main>
      </s.Wrapper>
    </ThemeProvider>
  );
};
Notification.displayName = 'notification';

export default Notification;
