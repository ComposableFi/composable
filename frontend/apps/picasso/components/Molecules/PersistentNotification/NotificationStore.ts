import create from "zustand";
import { devtools, persist } from "zustand/middleware";

export type NotificationVariant = "default" | "success" | "error";
export type Notification = {
  title: string;
  subtitle: string;
  actionButtonTitle?: string;
  actionButtonOnClick?: () => void;
  variant: NotificationVariant;
};
export type NotificationList = {
  [key in number]: Notification;
};

interface NotificationsState {
  list: NotificationList;
  active: Notification | null;
  notify: (notification: Notification) => number;
  archive: () => void;
  remove: (id: number) => void;
  toggleListVisibility: () => void;
  isListVisible: boolean;
  fire: (notification: Notification) => void;
}

export const useNotificationStore = create<NotificationsState>()(
  devtools(
    persist(
      (set) => ({
        list: {},
        active: null,
        fire: (notification: Notification) =>
          set((state) => ({
            ...state,
            active: {
              ...notification,
              variant: notification.variant ?? "default",
            },
          })),
        isListVisible: false,
        notify: (notification: Notification) => {
          const id = Date.now();

          set((state) => ({
            ...state,
            list: {
              ...state.list,
              ...{
                [id]: {
                  ...notification,
                  archived: false,
                },
              },
            },
          }));

          return id;
        },
        archive: () =>
          set((state) => {
            const id = Date.now();
            return {
              ...state,
              active: null,
              list: {
                ...state.list,
                [id]: {
                  ...state.active,
                },
              },
            } as NotificationsState;
          }),
        remove: (id: number) =>
          set((state) => {
            delete state.list[id];
            return state;
          }),
        toggleListVisibility: () =>
          set((state) => ({
            ...state,
            isListVisible: !state.isListVisible,
          })),
      }),
      {
        name: "notificationStore",
      }
    )
  )
);
