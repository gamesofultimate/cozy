import '@emotion/react';

declare module '@emotion/react' {
  export interface Theme {
    colors: {
      background: {
        main: string;
        box: string;
        mutedAccent: string;
      };
      basic: {
        white: string;
        black: string;
        gray0: string;
        gray1: string;
        gray4: string;
        gray6: string;
        orange: string;
        red: string;
        lightRed: string;
        green: string;
        darkBrown: string;
      };
      primary: {
        light: string;
        dark: string;
        highlight: string;
      };
    };
    fonts: {
      primary: string;
      secondary: string;
    };
  }
}
