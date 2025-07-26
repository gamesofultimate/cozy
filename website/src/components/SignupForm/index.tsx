import React from 'react';
import { Formik, Form } from 'formik';
//import { useSearchParams } from 'react-router-dom';

import styled from '@emotion/styled';

import Title from 'components/Title';
import Text from 'components/Text';
import Button, { ButtonKind } from 'components/Button';
import { useNotifications } from 'hooks/useNotifications';
import Alert from 'components/Alert';
import { useMutation } from 'hooks/useBackend';
import { Result } from '@ultimate-games/canvas';
import { Access } from 'types';
import { useLocalRef } from 'hooks/useCacheState';

type Signup = {
  username: string;
  password: string;
  confirmPassword: string;
  email: string;
};

export const Centered = styled.div(() => ({
  width: 300,
  margin: '0 auto',
}));

type SignupFormProps = {
  onSubmit: (access_token: string) => void;
};

const SignupForm: React.FC<SignupFormProps> = ({ onSubmit }) => {
  //const [] = useSearchParams();
  const [loading, signup] = useMutation<Result<Access, any>, Signup>('/signup');
  const [, setAccessToken] = useLocalRef<null | string>('settings.access-token', null);
  const [notifications, notify] = useNotifications();

  const initialValues = {
    username: '',
    password: '',
    confirmPassword: '',
    email: '',
  };

  const handleSubmit = async (values: Signup) => {
    console.log(values);
    const response = await signup(values);

    if (response?.Ok) {
      setAccessToken(response.Ok.access_token);
      onSubmit(response.Ok.access_token);
    } else {
      console.error(response?.Err);
      notify('Could not sign you up. Try again later.', 5000);
    }
  };

  return (
    <Centered>
      {notifications.map((notification) => (
        <Alert key={notification.id}>{notification.content}</Alert>
      ))}
      <Title>Sign-up</Title>
      <Formik<Signup> initialValues={initialValues} onSubmit={handleSubmit}>
        {() => (
          <Form>
            <Text name="username" label="Username" />
            <Text name="password" label="Password" type="password" />
            <Text name="confirmPassword" label="Confirm Password" type="password" />
            <Text name="email" label="E-mail" />
            {!loading && (
              <Button submit kind={ButtonKind.Large}>
                Sign-up
              </Button>
            )}
          </Form>
        )}
      </Formik>
    </Centered>
  );
};

export default SignupForm;
