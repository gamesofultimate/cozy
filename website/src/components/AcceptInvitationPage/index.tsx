import React, { useMemo } from 'react';
import { Formik, Form } from 'formik';
//import { useNavigate, useSearchParams } from 'react-router-dom';
import { useNavigate, useParams } from 'react-router-dom';

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
import { useQuery, useMutation } from 'hooks/useBackend';

import { Result, Invitation, Auth } from '@ultimate-games/canvas';
import { Accept, Access, GetInvitation } from 'types';
import Body from 'components/Body';

type AcceptForm = {
  username: string;
  password: string;
  confirm_password: string;
};

export const Centered = styled.div(() => ({
  width: 300,
  margin: '0 auto',
}));

const AcceptInvitationPage: React.FC = () => {
  const { invitation_token } = useParams();
  //const [] = useSearchParams();
  const navigate = useNavigate();
  const query = useMemo(() => {
    if (invitation_token) return { invitation_token };
    else return { invitation_token: 'unknown' };
  }, [invitation_token]);
  const [processing, accept] = useMutation<Result<Access, any>, Accept>('/accept-invitation');
  const [loading, invitationQuery] = useQuery<Result<[Invitation, Auth], any>, GetInvitation>(
    '/get-invitation',
    query
  );
  //const [loading, login] = useMutation<Result<Invitation, any>, GetInvitation>('/get-invitation');
  const [notifications, notify] = useNotifications();
  const [, setAccessToken] = useLocalRef<null | string>('settings.access-token', null);

  const initialValues = {
    username: '',
    password: '',
    confirm_password: '',
  };

  console.log(invitationQuery);

  const handleSubmit = async ({ username, password, confirm_password }: AcceptForm) => {
    if (!invitation_token) {
      return;
    }
    if (password !== confirm_password) {
      notify('Password confirmation does not match', 5000);
      return;
    }
    const response = await accept({ username, password, invitation_token });
    console.log(response);

    if (response?.Ok) {
      setAccessToken(response.Ok.access_token);
      window.location.href = '/';
    } else {
      notify('Could not accept your invitation. Try again.', 5000);
    }
  };

  const [, inviter] = invitationQuery?.Ok ?? [undefined, undefined];

  return (
    <Workspace>
      <Logo>
        <Image onClick={() => navigate('/')} source={Logo1x} retina={Logo2x} alt="Mark of the Deep's logo" />
      </Logo>
      <Right>
        <LoggedOutMenu />
      </Right>
      <Main>
        {loading ? (
          <Centered>
            <Title>Loading...</Title>
          </Centered>
        ) : (
          <Centered>
            {notifications.map((notification) => (
              <Alert key={notification.id}>{notification.content}</Alert>
            ))}
            <Title>Accept Invitation</Title>
            <div style={{ padding: '15px 0' }}>
              <Body>
                <strong>{inviter?.username ?? 'Unknown'}</strong> has invited you to play{' '}
                <strong>Mark of the Deep</strong>. Create your account with the form below.
              </Body>
            </div>
            <Formik<AcceptForm> initialValues={initialValues} onSubmit={handleSubmit}>
              {() => (
                <Form>
                  <Text name="username" label="Username" />
                  <Text name="password" label="Password" type="password" />
                  <Text name="confirm_password" label="Confirm password" type="password" />
                  <Button submit kind={ButtonKind.Large}>
                    {processing ? 'loading...' : 'Login'}
                  </Button>
                </Form>
              )}
            </Formik>
          </Centered>
        )}
      </Main>
    </Workspace>
  );
};

export default AcceptInvitationPage;
