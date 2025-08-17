#!/usr/bin/env bash
set -euo pipefail

# Config
URL="${URL:-http://localhost:4173/v1/graphql}"
USERNAME="${USERNAME:-rs}"
PASSWORD="${PASSWORD:-password123}"

echo "Using endpoint: $URL"
echo "Logging in as: $USERNAME"

# 1) Login to get token + refresh_token
echo
echo "==> LOGIN"
curl -sS -X POST "$URL" \
  -H 'content-type: application/json' \
  -d "$(jq -n --arg u "$USERNAME" --arg p "$PASSWORD" \
        '{query: "mutation ($input: LoginInput!){ login(input: $input){ success token refreshToken errors } }",
          variables: { input: { username: $u, password: $p } }}')" \
  | tee /tmp/login.json >/dev/null

TOKEN=$(jq -r '.data.login.token // empty' /tmp/login.json)
RT=$(jq -r '.data.login.refreshToken // empty' /tmp/login.json)

if [[ -z "${TOKEN}" || -z "${RT}" ]]; then
  echo "Login failed or empty token/refresh_token:"
  cat /tmp/login.json
  exit 1
fi

echo "Access token: $TOKEN"
echo "Refresh token: $RT"

# 2) Call me (should succeed)
echo
echo "==> ME (with valid token)"
curl -i -sS -X POST "$URL" \
  -H 'content-type: application/json' \
  -H "authorization: Bearer $TOKEN" \
  -d '{"query":"{ me { username firstName } }"}'
echo

# 3) Wait for token to expire (server issues 5s tokens)
echo
echo "Sleeping 8s to allow token to expire..."
sleep 8

# 4) Call me again (should be 401 TOKEN_EXPIRED)
echo
echo "==> ME (with expired token) - expecting 401"
curl -i -sS -X POST "$URL" \
  -H 'content-type: application/json' \
  -H "authorization: Bearer $TOKEN" \
  -d '{"query":"{ me { username firstName } }"}'
echo

# 5) Refresh token
echo
echo "==> REFRESH TOKEN"
curl -sS -X POST "$URL" \
  -H 'content-type: application/json' \
  -d "$(jq -n --arg rt "$RT" \
        '{query: "mutation ($input: RefreshInput!){ refreshToken(input: $input){ success token refreshToken errors } }",
          variables: { input: { refreshToken: $rt } }}')" \
  | tee /tmp/refresh.json >/dev/null

NEW_TOKEN=$(jq -r '.data.refreshToken.token // empty' /tmp/refresh.json)
NEW_RT=$(jq -r '.data.refreshToken.refreshToken // empty' /tmp/refresh.json)

if [[ -z "${NEW_TOKEN}" || -z "${NEW_RT}" ]]; then
  echo "Refresh failed or empty token/refresh_token:"
  cat /tmp/refresh.json
  exit 1
fi

echo "New access token: $NEW_TOKEN"
echo "New refresh token: $NEW_RT"

# 6) Call me with new token (should succeed)
echo
echo "==> ME (with refreshed token)"
curl -i -sS -X POST "$URL" \
  -H 'content-type: application/json' \
  -H "authorization: Bearer $NEW_TOKEN" \
  -d '{"query":"{ me { username firstName } }"}'
echo

# 7) Logout (invalidate refresh token)
echo
echo "==> LOGOUT"
curl -i -sS -X POST "$URL" \
  -H 'content-type: application/json' \
  -H "authorization: Bearer $NEW_TOKEN" \
  -d "$(jq -n --arg rt "$NEW_RT" \
        '{query: "mutation ($input: LogoutInput!){ logout(input: $input){ success } }",
          variables: { input: { refreshToken: $rt } }}')" 
# End of script
