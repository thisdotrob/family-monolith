// Temporary React type shims to unblock TS in mobileapp build. Remove when proper types resolve.
declare module 'react' {
  export type ReactNode = any;
  export type ComponentType<P = any> = any;
  export type FC<P = any> = (props: P) => any;
  export function useState<S = any>(
    initialState?: S | (() => S),
  ): [S, (value: S | ((prev: S) => S)) => void];
  export function useMemo<T = any>(factory: () => T, deps?: any[]): T;
  export function useEffect(effect: (...args: any[]) => any, deps?: any[]): void;
  export function useRef<T = any>(initialValue?: T | null): { current: T | null };
  export function createContext<T = any>(defaultValue: T): any;
  export function useContext<T = any>(ctx: any): T;
  export const Fragment: any;
  export function createElement(type: any, props?: any, ...children: any[]): any;
  const ReactDefault: any;
  export default ReactDefault;
}

declare module 'react/jsx-runtime' {
  export const jsx: any;
  export const jsxs: any;
  export const Fragment: any;
}
