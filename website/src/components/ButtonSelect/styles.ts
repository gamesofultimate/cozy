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
    display: 'flex',
    width: '100%',
    fontSize: 32,
    border: 0,
    boxSizing: 'border-box',
    grap: 10,
    justifyContent: 'space-between',
  },
]);

export const Label = styled.label(({ theme }) => [
  {
    position: 'absolute',
    bottom: 0,
    right: 25,
    fontSize: 11,
    fontFamily: theme.fonts.secondary,
    fontWeight: 200,
    color: '#000',
    textShadow: '1px 1px 3px #A3D9F8',
    ...padding(2, 4),
  },
]);

export const Input = styled.input(({ theme }) => [
  {
    display: 'block',
    width: '100%',
    fontSize: 44,
    background: 'transparent',
    color: theme.colors.basic.gray6,
    fontFamily: theme.fonts.primary,
    border: 0,
    padding: '10px 20px',
    textShadow: '1px 1px 3px #A3D9F8',
    boxSizing: 'border-box',
    '&::placeholder': {
      color: '#fff',
    },
  },
]);

export const Space = styled.div(({ theme }) => [
  {
    width: '48%',
    height: 'auto',
    fontSize: 32,
    borderRadius: 40,
    boxSizing: 'border-box',
  },
]);

export const Image = styled.img(({ theme }) => [
  {
    width: '100%',
    height: '100%',
    borderRadius: 40,
    boxShadow: '3px 3px 16px #a3d9f873',
    cursor: 'pointer',
  },
]);
