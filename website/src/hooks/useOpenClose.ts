import { useState } from 'react';

export const useOpenClose = (def = false) => {
  const [state, setter] = useState<boolean>(def);

  const close = () => {
    setter(false);
  };

  const open = () => {
    setter(true);
  };

  const toggle = () => {
    setter((curr) => !curr);
  };

  return {
    isOpen: state,
    close,
    open,
    toggle,
  };
};
