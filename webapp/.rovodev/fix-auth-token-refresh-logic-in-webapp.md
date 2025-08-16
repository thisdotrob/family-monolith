# Fix Auth Token Refresh Logic in Webapp

## Problem Description

The refresh logic is broken. When a token expires I am taken back to the LoginPage and both tokens are cleared from localstorage.

When this happens these are the network requests I see in devtools:

1. `query Me` request to `/v1/graphql/app`. This has the expired token in the headers and so gets a 401 response.
2. `mutation refreshToken` request to `/v1/graphql/auth`. This includes the original refreshToken from localstorage. Response is a 200 and includes a new auth token and a new refresh token.
3. `query Me` request to `/v1/graphql/app`. This still has the expired token in the headers and so gets a 401 response.
4. `mutation refreshToken` request to `/v1/graphql/auth`. This still has the original refreshToken, not the new one received from the first `mutation refreshToken` response, and so while the response is a 200 the response body has `refreshToken.success` of false and there are no new tokens included.
5. `query Me` request to `/v1/graphql/app`. This request has the new token in the header. Sometimes the request finishes and I see the HomePage and am still logged in as expected, but most of the time this request is marked as 'cancelled' in dev tools and I see the login page and am logged out as described above.

## Expected Flow

The flow should be:

1. `query Me` request with expired token, responds with 401 and "errors":[["TOKEN_EXPIRED"]] in response body.
2. `mutation RefreshToken`, responds with 200 and new token pair.
3. `query Me` request with refreshed token, i.e. attempt the original query again. HomePage should then show and user should still be logged in.

## Investigation and Fix Instructions

Can you investigate the code and fix the refresh logic so this doesn't happen? The key issues to address:

- Multiple refresh token requests being made with stale tokens
- New tokens not being properly stored/used after successful refresh
- Race conditions causing requests to be cancelled
- Proper handling of token expiration and refresh flow
- Ensuring the original query is retried with the new token after successful refresh
