import React from 'react';

import { padding } from 'polished';
import styled from '@emotion/styled';

export enum ButtonKind {
  Normal,
  Large,
}

type KindProps = {
  kind?: ButtonKind;
};

export const Link = styled.a<KindProps>(({ theme, kind = ButtonKind.Normal }) => [
  {
    border: '1px solid #fff',
    fontFamily: theme.fonts.primary,
    background: 'transparent',
    color: '#fff',
    margin: 0,
    lineHeight: 1,
    justifyItems: 'center',
    pointerEvents: 'auto',
    textDecoration: 'none',
    verticalAlign: 'center',
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center',
    gap: 20,
    cursor: 'pointer',
    textTransform: 'uppercase',
  },
  kind === ButtonKind.Normal && {
    fontSize: 13,
    ...padding(10),
  },
  kind === ButtonKind.Large && {
    fontSize: 32,
    padding: 10,
  },
]);


export const Button = styled.button<KindProps>(({ theme, kind = ButtonKind.Normal }) => [
  {
    border: 0,
    fontFamily: theme.fonts.primary,
    background: '#B64040',
    color: '#fff',
    margin: 0,
    lineHeight: 1,
    borderRadius: 40,
    padding: '0px 20px',
    justifyItems: 'center',
    pointerEvents: 'auto',
    textDecoration: 'none',
    verticalAlign: 'center',
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center',
    gap: 20,
    cursor: 'pointer',
    textTransform: 'uppercase',
  },
  kind === ButtonKind.Normal && {
    fontSize: 13,
    ...padding(10),
  },
  kind === ButtonKind.Large && {
    fontSize: 32,
    padding: 10,
  },
]);

type ButtonProps = {
  children: React.ReactNode;
  href?: string;
  target?: string;
  rel?: string;
  onClick?: () => void;
  submit?: boolean;
};
const Main: React.FC<ButtonProps & KindProps> = ({ children, href, target, rel, submit, kind, onClick }) => {
  if (href) {
    return (
      <Link kind={kind} onClick={onClick} href={href} target={target} rel={rel}>
        {children}
      </Link>
    );
  } else {
    return (
      <Button type={submit ? 'submit' : undefined} kind={kind} onClick={onClick}>
        {children}
      </Button>
    );
  }
};

export default Main;
