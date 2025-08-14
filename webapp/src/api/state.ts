import { makeVar } from '@apollo/client';

export const isRefreshingTokenVar = makeVar<boolean>(false);
