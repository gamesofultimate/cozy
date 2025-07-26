import React from 'react';
import { Formik, Form } from 'formik';
//import { useNavigate, useSearchParams } from 'react-router-dom';
import { useNavigate } from 'react-router-dom';

import Workspace, { Logo, Main, Right } from 'components/Workspace';
import { useLocalRef } from 'hooks/useCacheState';
import { useNotifications } from 'hooks/useNotifications';

import Logo1x from 'images/logo@1x.png';
import Logo2x from 'images/logo@2x.png';

import styled from '@emotion/styled';

import Alert from 'components/Alert';
import LoggedOutMenu from 'components/LoggedOutMenu';
import Title from 'components/Title';
import Text from 'components/Text';
import Button, { ButtonKind } from 'components/Button';
import Image from 'components/Image';
import { useMutation } from 'hooks/useBackend';

import { Result } from '@ultimate-games/canvas';
import { Access } from 'types';

type LoginForm = {
  username: string;
  password: string;
};

export const Centered = styled.div(() => ({
  width: 300,
  margin: '0 auto',
}));

const LoginPage: React.FC = () => {
  //const [] = useSearchParams();
  const navigate = useNavigate();
  const [loading, login] = useMutation<Result<Access, any>, LoginForm>('/login');
  const [notifications, notify] = useNotifications();
  const [, setAccessToken] = useLocalRef<null | string>('settings.access-token', null);

  const initialValues = {
    username: '',
    password: '',
  };

  const handleSubmit = async (data: LoginForm) => {
    const response = await login(data);

    if (response?.Ok) {
      setAccessToken(response.Ok.access_token);
      window.location.href = '/';
    } else {
      notify('Could not log you in. Try again.', 5000);
    }
  };

  return (
    <Workspace>
      <Logo>
        <Image onClick={() => navigate('/')} source={Logo1x} retina={Logo2x} alt="Mark of the Deep's logo" />
      </Logo>
      <Right>
        <LoggedOutMenu />
      </Right>
      <Main>
        <Centered>
          {notifications.map((notification) => (
            <Alert key={notification.id}>{notification.content}</Alert>
          ))}
          <Title>Login</Title>
          <Formik<LoginForm> initialValues={initialValues} onSubmit={handleSubmit}>
            {() => (
              <Form>
                <Text name="username" label="Username" />
                <Text name="password" label="Password" type="password" />
                <Button submit kind={ButtonKind.Large}>
                  {loading ? 'loading...' : 'Login'}
                </Button>
              </Form>
            )}
          </Formik>
        </Centered>
      </Main>
    </Workspace>
  );
};

export default LoginPage;
