import React from 'react';
import styled from '@emotion/styled';

const Path = styled.path(() => ({
  fill: 'currentcolor',
}));

const Success: React.FC = () => (
  <svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
    <Path
      d="M6 0C2.688 0 0 2.688 0 6C0 9.312 2.688 12 6 12C9.312 12 12 9.312 12 6C12 2.688 9.312 0 6 0ZM6 10.8C3.354 10.8 1.2 8.646 1.2 6C1.2 3.354 3.354 1.2 6 1.2C8.646 1.2 10.8 3.354 10.8 6C10.8 8.646 8.646 10.8 6 10.8ZM9.17849 3.77551C8.94429 3.53965 8.56303 3.53897 8.328 3.774L4.8 7.302L3.66941 6.17577C3.43543 5.9427 3.05693 5.94307 2.82341 6.17659C2.58957 6.41043 2.58957 6.78957 2.82341 7.02341L4.8 9L9.177 4.623C9.41086 4.38914 9.41153 4.01019 9.17849 3.77551Z"
      fill="#BDBDBD"
    />
  </svg>
);

export default Success;
