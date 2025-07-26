export type Access = {
  access_token: string;
};

export type GetAchievements = {
  unique_id: string;
};

export type GetRanking = {
  unique_id: string;
};

export type GetGlobalRanking = {};

export type GetGlobalMoments = {};

export type GetMoments = {
  unique_id: string;
};

export type GetMoment = {
  moment_id: string;
};

export type GetPlaySession = {
  session_id: string;
};

export type GetInvitation = {
  invitation_token: string;
};

export type Accept = {
  username: string;
  password: string;
  invitation_token: string;
};

export type Invite = {
  access_token: string;
  email: string;
};

export type GetOrCreateSession = {
  unique_key: string;
  branch: string;
};
