import { createRenderer, RenderSender } from '@ultimate-games/canvas';
import { FocusState, WorkspaceMode } from 'components/Workspace';
import { makeAutoObservable } from 'mobx';

import type {
  Message,
  GameplayStats,
  // @ts-ignore
} from 'types/ultimate';

type Website = {
  focused: FocusState;
  mode: WebsiteMode;
};

type Config = {
  debouncingEscape: boolean;
};

type Stats = {
}

export enum WebsiteMode {
  Normal,
  SigningUp,
  Inviting,
  Wishlist,
  Pause,
  OutOfCapacity,
}

const defaultWebsite = (): Website => {
  return {
    focused: FocusState.Unfocused,
    mode: WebsiteMode.Normal,
  };
};

const defaultConfig = (): Config => {
  return {
    debouncingEscape: false,
  };
};

const defaultStats = (): Stats => {
  return { };
};

export class Game {
  website: Website;
  config: Config;
  stats: GameplayStats;

  constructor() {
    this.website = defaultWebsite();
    this.config = defaultConfig();
    this.stats = defaultStats();

    makeAutoObservable(this);
  }

  receiveFromRenderer(message: Message) {
    //console.log('message', message, { ...this.website });
    if (message === 'StartGame') {
      this.website.focused = FocusState.Focused;
      this.website.mode = WebsiteMode.Normal;
    } else if (message === 'StopGame' && this.invitationDialog && !this.config.debouncingEscape) {
      console.log('closing invite');
      this.config.debouncingEscape = true;
      setTimeout(() => {
        this.config.debouncingEscape = false;
      }, 750);
    } else if (message === 'StopGame' && !this.config.debouncingEscape) {
      const activeElement = document.activeElement as HTMLElement;
      activeElement?.blur();

      console.log('regular stop');
      this.website.focused = FocusState.Unfocused;
      this.website.mode = WebsiteMode.Pause;
      setTimeout(() => {
        this.config.debouncingEscape = false;
      }, 750);
    } else if (message === 'TriggerInvitation') {
      this.website.mode = WebsiteMode.Inviting;
    }
    else if ("FinishGame" in message) {
      this.stats = message.FinishGame;
      this.website.mode = WebsiteMode.Wishlist;
      this.website.focused = FocusState.Unfocused;

      if (document.fullscreenElement) {
        document.exitFullscreen();
      }
    }
  }

  get hasGun(): boolean {
    return this.stats.hasGun;
  }

  get hasHookshot(): boolean {
    return this.stats.hasGun;
  }

  get isSigningUp(): boolean {
    return this.website.mode === WebsiteMode.SigningUp;
  }

  toggleWishlist() {
    /*
    if (this.website.mode === WebsiteMode.Normal) {
      this.website.mode = WebsiteMode.Wishlist;
    } else {
      this.website.mode = WebsiteMode.Normal;
    }
   */
  }

  setOutOfCapacity() {
    this.website.mode = WebsiteMode.OutOfCapacity;
  }

  toggleSignup() {
    if (this.website.mode === WebsiteMode.Normal) {
      this.website.mode = WebsiteMode.SigningUp;
    } else {
      this.website.mode = WebsiteMode.Normal;
    }
  }

  openSignup() {
    this.website.mode = WebsiteMode.SigningUp;
  }

  finishSignup() {
    this.website.mode = WebsiteMode.Normal;
  }

  openInvitationDialog() {
    this.website.mode = WebsiteMode.Inviting;
  }

  closeInvitationDialog() {
    this.website.mode = WebsiteMode.Normal;
  }

  closePauseDialog() {
    this.website.mode = WebsiteMode.Normal;
    this.website.focused = FocusState.Focused;

    const canvas = document.getElementById('canvas');
    canvas?.focus();

    sendToGame('StartGame');
  }

  closeWishlistDialog() {
    this.website.mode = WebsiteMode.Normal;
    sendToGame('ReloadGame');
  }

  closeOutOfCapacityDialog() {
    this.website.mode = WebsiteMode.Normal;
  }

  sendShareEvent() {
    sendToGame('Shared');
  }

  setDeveloperMode(key: string) {
    sendToGame({ Developer: key });
  }

  get focused(): FocusState {
    return this.website.focused;
  }

  get mode(): WebsiteMode {
    return this.website.mode;
  }

  get invitationDialog(): boolean {
    return this.website.mode === WebsiteMode.Inviting;
  }

  get outOfCapacityDialog(): boolean {
    return this.website.mode === WebsiteMode.OutOfCapacity;
  }

  get wishlistDialog(): boolean {
    return this.website.mode === WebsiteMode.Wishlist;
  }

  get pauseDialog(): boolean {
    return this.website.mode === WebsiteMode.Pause;
  }

  get workspaceMode(): WorkspaceMode {
    switch (this.website.mode) {
      case WebsiteMode.Normal: {
        return WorkspaceMode.Working;
      }
      case WebsiteMode.SigningUp: {
        return WorkspaceMode.UserInput;
      }
      case WebsiteMode.Inviting: {
        return WorkspaceMode.UserInput;
      }
      case WebsiteMode.Wishlist: {
        return WorkspaceMode.UserInput;
      }
      default: {
        return WorkspaceMode.Working;
      }
    }
  }
}

const game = new Game();
const renderer = ['game'] as const;
const Manager = createRenderer<Message, Game>(game, renderer);
export const sendToGame = Manager.senders['game'];
export const gameBus = Manager.buses['game'];
export const GameProvider = Manager.Provider;
export const MockProvider = Manager.MockProvider;
export const useGameData = Manager.useRenderer;
export type Sender = RenderSender<Message>;
