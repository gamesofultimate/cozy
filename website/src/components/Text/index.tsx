import React, { useCallback } from 'react';
import { useField } from 'formik';

import * as s from './styles';

export type PublicProps = {
  autocomplete?: HTMLInputElement['autocomplete'];
  disabled?: HTMLInputElement['disabled'];
  label?: string;
  maxLength?: number;
  name: string;
  placeholder?: HTMLInputElement['placeholder'];
  readonly?: HTMLInputElement['readOnly'];
  tabIndex?: number;
  type?: 'text' | 'password';
};

type ControlledProps = PublicProps & {
  error?: string | null;
  onBlur?: React.FocusEventHandler<HTMLInputElement>;
  onChange?: React.ChangeEventHandler<HTMLInputElement>;
  placeholder?: string;
  rows?: number;
  value: string;
};

export const Controlled: React.FunctionComponent<ControlledProps> = ({
  autocomplete,
  disabled,
  error,
  label,
  maxLength,
  name,
  onBlur,
  onChange,
  placeholder,
  readonly,
  tabIndex,
  type = 'text',
  value,
}) => {
  return (
    <s.Field>
      <s.Wrapper>
        {label && <s.Label>{label}</s.Label>}
        <s.Input
          id={name}
          autoComplete={autocomplete}
          disabled={disabled}
          maxLength={maxLength}
          onBlur={onBlur}
          onChange={onChange}
          placeholder={placeholder}
          readOnly={readonly}
          tabIndex={tabIndex}
          type={type}
          value={value}
        />
      </s.Wrapper>
      {error && <s.ErrorMessage>{error}</s.ErrorMessage>}
    </s.Field>
  );
};

const Public: React.FunctionComponent<PublicProps> = ({ name, ...rest }) => {
  const [field, meta, helper] = useField(name);

  const handleOnBlur = useCallback(
    (event: any) => {
      if (field.onBlur) {
        field.onBlur(event);
      }
    },
    [field]
  );

  const handleOnChange = useCallback(
    (event: any) => {
      helper.setValue(event.target.value);
    },
    [helper]
  );

  return (
    <Controlled
      {...rest}
      error={meta.touched && meta.error ? meta.error : null}
      name={name}
      onBlur={handleOnBlur}
      onChange={handleOnChange}
      value={field.value}
    />
  );
};

export default Public;
