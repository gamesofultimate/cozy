import { useState, useRef, useCallback, useEffect } from 'react';

type Getter<T> = (path?: string[]) => T | null;
type GetterWithDefault<T> = (defaultValue: T, ...path: string[]) => T;
type Setter<T> = (data: T, ...path: string[]) => void;

type Manager<T> = {
  get: Getter<T>;
  getWithDefault: GetterWithDefault<T>;
  set: Setter<T>;
};

/// allow you to manage local storage in a typesafe manner
export const localstateManager = <T>(prepath: string[]): Manager<T> => {
  const getter = (path: string[] = []) => {
    const key = [...prepath, ...path].join('.');
    const state = localStorage.getItem(key);
    return state ? JSON.parse(state) : null;
  };

  const getWithDefault = (defaultValue: T, ...path: string[]): T => {
    const key = [...prepath, ...path].join('.');
    const state = localStorage.getItem(key);
    return state ? JSON.parse(state) : defaultValue;
  };

  const setter = (data: T, ...path: string[]) => {
    const key = [...prepath, ...path].join('.');
    localStorage.setItem(key, JSON.stringify(data));
  };

  return { get: getter, getWithDefault, set: setter };
};

export const useCache = <T>(fn: () => T, cache?: any[]): T | null => {
  const [data, mutator] = useState<T | null>(null);

  useEffect(() => {
    mutator(fn());
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, cache);

  return data;
};

export const useLocalState = <T>(key: string, defaultValue: T) => {
  const state = localStorage.getItem(key);
  const value = state ? JSON.parse(state) : defaultValue;
  const [data, mutator] = useState<T>(value);

  const mutate = useCallback((newData: T) => {
    if (newData === null || newData === undefined) {
      localStorage.removeItem(key);
    } else {
      localStorage.setItem(key, JSON.stringify(newData));
    }

    mutator(newData);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return [data, mutate] as [T, (data: T) => void];
};

export const useLocalRef = <T>(key: string, defaultValue: T) => {
  const state = localStorage.getItem(key);

  if (!state) localStorage.setItem(key, JSON.stringify(defaultValue));

  const value = state ? JSON.parse(state) : defaultValue;
  const reference = useRef<T>(value);

  const mutate = (newData: T) => {
    if (newData === null || newData === undefined) {
      localStorage.removeItem(key);
    } else {
      localStorage.setItem(key, JSON.stringify(newData));
    }

    reference.current = newData;
  };

  return [reference.current, mutate] as [T, (data: T) => void];
};

export const useSessionState = <T>(key: string, defaultValue: T) => {
  const state = sessionStorage.getItem(key);
  const value = state ? JSON.parse(state) : defaultValue;
  const [data, mutator] = useState<T>(value);

  const mutate = (newData: T) => {
    if (newData === null || newData === undefined) {
      sessionStorage.removeItem(key);
    } else {
      sessionStorage.setItem(key, JSON.stringify(newData));
    }

    mutator(newData);
  };

  return [data, mutate] as [T, (data: T) => void];
};

export const useSessionRef = <T>(key: string, defaultValue: T) => {
  const state = sessionStorage.getItem(key);
  const value = state ? JSON.parse(state) : defaultValue;
  const reference = useRef<T>(value);

  const mutate = (newData: T) => {
    if (newData === null || newData === undefined) {
      sessionStorage.removeItem(key);
    } else {
      sessionStorage.setItem(key, JSON.stringify(newData));
    }

    reference.current = newData;
  };

  return [reference.current, mutate] as [T, (data: T) => void];
};
