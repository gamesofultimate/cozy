import React from 'react';
import styled from '@emotion/styled';
import { padding, lighten } from 'polished';

import InfoIcon from 'svgs/Info';
import SuccessIcon from 'svgs/Success';
import WarningIcon from 'svgs/Warning';
import FailureIcon from 'svgs/Failure';

export enum BannerType {
  INFO,
  SUCCESS,
  WARNING,
  FAILURE,
}

type BannerProps = {
  bannerType?: BannerType;
};

const Banner = styled.div<BannerProps>(({ theme, bannerType }) => [
  {
    borderRadius: 4,
    transition: 'color 300ms ease-in-out,background 300ms ease-in-out',
    ...padding(10, 20),
  },
  bannerType === BannerType.INFO && {
    background: lighten(0.4, theme.colors.basic.gray0),
    color: theme.colors.basic.gray0,
  },
  bannerType === BannerType.SUCCESS && {
    background: lighten(0.4, theme.colors.basic.green),
    color: theme.colors.basic.green,
  },
  bannerType === BannerType.WARNING && {
    background: lighten(0.4, theme.colors.basic.orange),
    color: theme.colors.basic.orange,
  },
  bannerType === BannerType.FAILURE && {
    background: lighten(0.4, theme.colors.basic.red),
    color: theme.colors.basic.red,
  },
]);

const Copy = styled.span(({ theme }) => ({
  fontFamily: theme.fonts.secondary,
  fontWeight: 400,
  fontSize: 16,
  lineHeight: 1,
  marginLeft: 5,
}));

type AlertProps = BannerProps & {
  children: string;
};

const Alert: React.FC<AlertProps> = ({ children, bannerType = BannerType.INFO }) => {
  return (
    <Banner bannerType={bannerType}>
      {bannerType === BannerType.INFO && <InfoIcon />}
      {bannerType === BannerType.SUCCESS && <SuccessIcon />}
      {bannerType === BannerType.WARNING && <WarningIcon />}
      {bannerType === BannerType.FAILURE && <FailureIcon />}
      <Copy>{children}</Copy>
    </Banner>
  );
};

export default Alert;
