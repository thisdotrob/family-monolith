const LocalStorage = {
  setItem: async (key: string, value: string) => localStorage.setItem(key, value),
  getItem: async (key: string): Promise<string | null> => localStorage.getItem(key),
  removeItem: async (key: string) => localStorage.removeItem(key),
};

export default LocalStorage;
