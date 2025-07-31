import { createRenderer, RenderSender } from '@ultimate-games/canvas';
import { FocusState, WorkspaceMode } from 'components/Workspace';
import { makeAutoObservable } from 'mobx';

import type {
  Message,
  GameplayStats,
  InventoryItem,
  StateMachine,
  // @ts-ignore
} from 'types/ultimate';

type Website = {
  focused: FocusState;
  mode: WebsiteMode;
  download_percent: number;
};

type Config = {
  debouncingEscape: boolean;
};

type Ui = {
  inventory: InventoryItem[],
  mode: UiMode,
  cash: number;
  rest: number;
  social: number;
  hunger: number;
  current_action: string;
}

type Stats = {
}

export enum UiMode {
  Hidden,
  Small,
  Expanded,
  Signup,
}

export enum WebsiteMode {
  Downloading,
  Normal,
  SigningUp,
  Inviting,
  Wishlist,
  Pause,
  OutOfCapacity,
  SalesDialog,
}

const defaultUi = (): Ui => {
  const inventory: InventoryItem[] = [...new Array(15)].map(_ => ({
    item: "Nothing",
    quantity: "Empty",
  }));

  //inventory[0] = {item:{Seed:"Pumpkin"},quantity:{Finite:6}};

  return {
    inventory,
    mode: UiMode.Hidden,
    // mode: UiMode.Small,
    cash: 0,
    rest: 1,
    social: 1,
    hunger: 1,
    current_action: 'Starting',
  }
}

const defaultWebsite = (): Website => {
  return {
    focused: FocusState.Unfocused,
    //focused: FocusState.Focused,
    mode: WebsiteMode.Normal,
    //mode: WebsiteMode.Downloading,
    //mode: WebsiteMode.SalesDialog,
    //mode: WebsiteMode.Pause,

    download_percent: 0,
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

const defaultStateMachine = (): StateMachine => {
  return {
    players: [],
    state: "Initializing",
    ready: {},
    admin: null,
  };
};

export class Game {
  ui: Ui;
  website: Website;
  config: Config;
  stats: GameplayStats;
  machine: StateMachine;

  constructor() {
    this.website = defaultWebsite();
    this.config = defaultConfig();
    this.stats = defaultStats();
    this.ui = defaultUi();
    this.machine = defaultStateMachine();

    makeAutoObservable(this);
  }

  receiveFromRenderer(message: Message) {
    //console.log('message', message, { ...this.website });
    if (message === 'StartGame') {
      this.website.focused = FocusState.Focused;
      this.website.mode = WebsiteMode.Normal;
      this.ui.mode = UiMode.Small;
    } else if (message === 'StopGame' && this.invitationDialog && !this.config.debouncingEscape) {
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
      this.ui.mode = UiMode.Hidden;

      setTimeout(() => {
        this.config.debouncingEscape = false;
      }, 750);
    } else if (message === 'TriggerInvitation') {
      this.website.mode = WebsiteMode.Inviting;
    }
    else if ("StartSale" in message) {
      this.website.mode = WebsiteMode.SalesDialog;
    }
    else if ("FinishGame" in message) {
      this.stats = message.FinishGame;
      this.website.mode = WebsiteMode.Wishlist;
      this.website.focused = FocusState.Unfocused;

      if (document.fullscreenElement) {
        document.exitFullscreen();
      }
    }
    else if ("UpdateStateMachine" in message) {
      this.machine = message.UpdateStateMachine.state;
    }
    else if ("UpdateCharacter" in message) {
      if (!message.UpdateCharacter) return;

      const rest = message.UpdateCharacter.character.rest;
      const social = message.UpdateCharacter.character.social;
      const hunger = message.UpdateCharacter.character.hunger;

      //console.log(message.UpdateCharacter);
      this.ui.inventory = message.UpdateCharacter.character.inventory;
      this.ui.cash = message.UpdateCharacter.character.cash;
      this.ui.current_action = message.UpdateCharacter.state;
      this.ui.rest = rest.current / rest.max;
      this.ui.social = social.current / social.max;
      this.ui.hunger = hunger.current / hunger.max;
    }
    else if ("UpdateDownloadStats" in message) {
      const pendingRequired = message.UpdateDownloadStats.pending_required;
      const pendingPriority = message.UpdateDownloadStats.pending_priority;
      const downloadedRequired = message.UpdateDownloadStats.downloaded_required;
      const downloadedPriority = message.UpdateDownloadStats.downloaded_priority;
      const total = (pendingPriority + pendingRequired + downloadedRequired + downloadedPriority) ?? 1;
      const downloaded = downloadedRequired + downloadedPriority;
      const percent = downloaded / total;
      console.log('download', message.UpdateDownloadStats, `${downloaded} / ${total} = ${percent * 100}`);
      this.website.download_percent = percent;
      if (percent >= 1) {
        this.website.mode = WebsiteMode.Normal;
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

  finishSales() {
    this.website.mode = WebsiteMode.Normal;
  }

  openInvitationDialog() {
    this.website.mode = WebsiteMode.Inviting;
  }

  closeInvitationDialog() {
    this.website.mode = WebsiteMode.Normal;
  }

  restartGame() {
    this.website.mode = WebsiteMode.Normal;
    this.website.focused = FocusState.Focused;
    this.ui.mode = UiMode.Small;

    const canvas = document.getElementById('canvas');
    canvas?.focus();

    sendToGame('StartGame');
  }

  pressPlay() {
    this.website.mode = WebsiteMode.Normal;
    this.website.focused = FocusState.Focused;
    this.ui.mode = UiMode.Small;
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

  finishSignup() {
    this.website.mode = WebsiteMode.Normal;
    sendToGame('FinishSignup');
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

  get isSelling(): boolean {
    return this.website.mode === WebsiteMode.SalesDialog;
  }

  get wishlistDialog(): boolean {
    return this.website.mode === WebsiteMode.Wishlist;
  }

  get pauseDialog(): boolean {
    return false;
    //return this.website.mode === WebsiteMode.Pause;
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
