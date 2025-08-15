// A simple in-memory store to share the refresh state
// between the Apollo link and the AuthContext.

type Listener = (isRefreshing: boolean) => void;

let isRefreshing = false;
const listeners: Listener[] = [];

export const refreshTokenStateManager = {
  set: (refreshing: boolean) => {
    isRefreshing = refreshing;
    listeners.forEach(listener => listener(isRefreshing));
  },
  subscribe: (listener: Listener) => {
    listeners.push(listener);
    // Return an unsubscribe function
    return () => {
      const index = listeners.indexOf(listener);
      if (index > -1) {
        listeners.splice(index, 1);
      }
    };
  },
  get: () => isRefreshing,
};
