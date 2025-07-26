import React from 'react';
import styled from '@emotion/styled';

const Path = styled.path(() => ({
  fill: 'currentcolor',
}));

const Success: React.FC = () => (
  <svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
    <Path
      d="M6.6 9L5.4 9L5.4 7.8L6.6 7.8L6.6 9ZM6.6 6C6.6 6.33137 6.33137 6.6 6 6.6C5.66863 6.6 5.4 6.33137 5.4 6L5.4 3.6C5.4 3.26863 5.66863 3 6 3C6.33137 3 6.6 3.26863 6.6 3.6L6.6 6ZM6 12C9.312 12 12 9.312 12 6C12 2.688 9.312 -2.34992e-07 6 -5.24537e-07C2.688 -8.14081e-07 8.14081e-07 2.688 5.24537e-07 6C2.34992e-07 9.312 2.688 12 6 12ZM6 1.2C8.646 1.2 10.8 3.354 10.8 6C10.8 8.646 8.646 10.8 6 10.8C3.354 10.8 1.2 8.646 1.2 6C1.2 3.354 3.354 1.2 6 1.2Z"
      fill="#BDBDBD"
    />
  </svg>
);

export default Success;
