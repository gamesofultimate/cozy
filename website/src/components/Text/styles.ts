import styled from '@emotion/styled';
import { padding, transparentize } from 'polished';

export const ErrorMessage = styled.div(({ theme }) => [
  {
    color: theme.colors.basic.red,
    fontSize: 11,
  },
]);

export const Field = styled.div(() => ({
  width: '100%',
  fontFamily: 'Open Sans',

  alignItems: 'baseline',
  gap: '10px',
  ...padding(10, 0),
}));

export const Wrapper = styled.div(({ theme }) => [
  {
    position: 'relative',
    display: 'block',
    width: '100%',
    fontSize: 32,
    borderRadius: 4,
    background: transparentize(0.2, theme.colors.basic.gray1),
    border: '1px solid #000',
    boxSizing: 'border-box',
  },
]);

export const Label = styled.label(({ theme }) => [
  {
    position: 'absolute',
    top: 0,
    right: 0,
    fontSize: 11,
    fontFamily: theme.fonts.secondary,
    fontWeight: 200,
    color: theme.colors.basic.gray4,
    ...padding(2, 4),
  },
]);

export const Input = styled.input(({ theme }) => [
  {
    display: 'block',
    width: '100%',
    fontSize: 22,
    background: 'transparent',
    color: theme.colors.basic.gray6,
    fontFamily: theme.fonts.primary,
    border: 0,
    padding: '10px 20px',
    boxSizing: 'border-box',
    '&::placeholder': {
      color: '#fff',
    },
  },
]);
