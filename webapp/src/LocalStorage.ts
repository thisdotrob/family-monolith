const LocalStorage = {
  setItem: async (key, value) => localStorage.setItem(key, value),
  getItem: async (key) => localStorage.getItem(key),
  removeItem: async (key) => localStorage.removeItem(key),
};

export default LocalStorage
