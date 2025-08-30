// Minimal React type shims to satisfy tsc in this monorepo environment
// If proper @types/react are installed, these will be ignored by module resolution precedence.
declare module 'react' {
  export type ReactNode = any;

  export interface Context<T> { Provider: any; Consumer: any; }

  export function createContext<T>(defaultValue: T | null): Context<T>;
  export function useContext<T>(ctx: Context<T>): T;

  export function useState<T>(initialState: T): [T, (v: T) => void];
  export function useEffect(effect: any, deps?: any[]): void;
  export function useMemo<T>(factory: () => T, deps: any[]): T;

  const React: any;
  export default React;
}

declare module 'react/jsx-runtime' {
  export const jsx: any;
  export const jsxs: any;
  export const Fragment: any;
}

declare namespace JSX {
  interface IntrinsicElements {
    [elem: string]: any;
  }
}
