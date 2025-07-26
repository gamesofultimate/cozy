// This is the react-hook-mousetrap library. The version available on npm was broken, so I pulled
// it into the codebase for now.
// TODO: check if the version on npm is useable again
import mousetrap from 'mousetrap';
import { useEffect, useRef } from 'react';

/**
 * Use mousetrap hook
 *
 * @param  {(string | string[])} handlerKey - A key, key combo or array of combos according to Mousetrap documentation.
 * @param  { function } handlerCallback - A function that is triggered on key combo catch.
 */

const useMousetrap = (
  handlerKey: string | string[],
  handlerCallback: (e: Mousetrap.ExtendedKeyboardEvent) => void,
  eventType?: string
) => {
  const actionRef = useRef(null);
  // @ts-ignore
  actionRef.current = handlerCallback;

  useEffect(() => {
    mousetrap.bind(
      handlerKey,
      // @ts-ignore
      (e) => typeof actionRef.current === 'function' && actionRef.current(e),
      eventType
    );
    return () => {
      mousetrap.unbind(handlerKey);
    };
  }, [handlerKey, eventType]);
};

export default useMousetrap;
