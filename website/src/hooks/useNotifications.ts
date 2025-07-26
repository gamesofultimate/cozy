import { useState } from 'react';
import * as uuid from 'uuid';

type Event = {
  content: string;
  timeout: number;
};

type Notification = Event & {
  id: string;
};

type Notifications = Notification[];

type NotificationManager = [Notifications, (content: string, timeout: number) => void];

export const useNotifications = (): NotificationManager => {
  const [data, setter] = useState<Notifications>([]);

  const creator = (content: string, timeout: number) => {
    const id = uuid.v4();
    const newNotification = { id, content, timeout };
    setter((prev: Notification[]) => [...prev, newNotification]);

    setTimeout(() => {
      setter((prev: Notification[]) => {
        const filtered = prev.filter((item) => item.id !== id);
        return filtered;
      });
    }, timeout);
  };

  return [data, creator];
};
