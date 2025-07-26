import React from 'react';
import styled from '@emotion/styled';

const Path = styled.path(() => ({
  fill: 'currentcolor',
}));

const Info: React.FC = () => (
  <svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
    <Path
      d="M5.4 3H6.6V4.2H5.4V3ZM5.4 6C5.4 5.66863 5.66863 5.4 6 5.4C6.33137 5.4 6.6 5.66863 6.6 6V8.4C6.6 8.73137 6.33137 9 6 9C5.66863 9 5.4 8.73137 5.4 8.4V6ZM6 0C2.688 0 0 2.688 0 6C0 9.312 2.688 12 6 12C9.312 12 12 9.312 12 6C12 2.688 9.312 0 6 0ZM6 10.8C3.354 10.8 1.2 8.646 1.2 6C1.2 3.354 3.354 1.2 6 1.2C8.646 1.2 10.8 3.354 10.8 6C10.8 8.646 8.646 10.8 6 10.8Z"
      fill="#BDBDBD"
    />
  </svg>
);

export default Info;
