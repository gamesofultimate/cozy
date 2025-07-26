import { useCallback, useEffect, useState } from 'react';

export const useMutation = <T, B>(path: string): [boolean, (body?: B) => Promise<T | null>] => {
  const [loading, setState] = useState<boolean>(false);
  const mutator = async (body?: any): Promise<T | null> => {
    try {
      setState(true);

      const url = `${process.env.REACT_APP_BACKEND_URL}${path}`;

      const options = {
        method: 'post',
      };

      // @ts-ignore I could probably type this better, but speed is of the essence today
      if (body) options.body = JSON.stringify(body);

      const request = await fetch(url, options);
      const response = (await request.json()) as T;

      setState(false);

      return response;
    } catch (e) {
      setState(false);
      return null;
    }
  };

  return [loading, mutator];
};

export const useQuery = <T, B>(path: string, body?: B): [boolean, T | null, () => void] => {
  const [[loading, data], setState] = useState<[boolean, null | T]>([true, null]);
  const effect = useCallback(() => {
    const url = `${process.env.REACT_APP_BACKEND_URL}${path}`;
    const effect = async () => {
      const options = {
        method: 'post',
      };

      // @ts-ignore I could probably type this better, but speed is of the essence today
      if (body) options.body = JSON.stringify(body);

      const request = await fetch(url, options);
      const response = (await request.json()) as T;

      setState([false, response]);
    };
    effect();
  }, [path, body]);

  useEffect(() => {
    effect();
  }, [path, body, effect]);

  return [loading, data, effect];
};

export const useTrigger = <T, B>(
  path: string,
  executeQuery = true
): [boolean, T | null, (body?: B) => void] => {
  const [[loading, data], setState] = useState<[boolean, null | T]>([true, null]);
  console.log(path, executeQuery);
  const query = useCallback(
    (body?: B) => {
      const url = `${process.env.REACT_APP_BACKEND_URL}${path}`;
      const effect = async () => {
        const options = {
          method: 'post',
        };

        // @ts-ignore I could probably type this better, but speed is of the essence today
        if (body) options.body = JSON.stringify(body);

        const request = await fetch(url, options);
        const response = (await request.json()) as T;

        setState([false, response]);
      };
      if (executeQuery) {
        effect();
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [executeQuery]
  );

  return [loading, data, query];
};

export const useGet = <T>(path: string): [boolean, T | null] => {
  const [[loading, data], setState] = useState<[boolean, null | T]>([true, null]);
  useEffect(() => {
    const url = `${process.env.REACT_APP_BACKEND_URL}${path}`;
    const effect = async () => {
      const options = {
        method: 'get',
      };

      const request = await fetch(url, options);
      const response = (await request.json()) as T;

      setState([false, response]);
    };
    effect();
  }, [path]);

  return [loading, data];
};
