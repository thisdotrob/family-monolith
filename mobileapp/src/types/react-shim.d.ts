// Minimal React and JSX shims for mobile typecheck without @types/react
// These are permissive and only intended to allow tsc to run in CI when node_modules types may be absent.

declare module 'react' {
  export type ReactNode = any;
  export type PropsWithChildren<P = {}> = P & { children?: ReactNode };
  export type ComponentProps<T extends any> = any;
  export type ReactElement = any;
  export interface Context<T> { Provider: any; Consumer: any }
  export interface FC<P = {}> { (props: PropsWithChildren<P>): ReactElement | null }

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
  interface IntrinsicElements { [elem: string]: any }
}
