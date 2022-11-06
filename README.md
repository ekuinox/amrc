# oauth2-helper

## 使い方

```console
> cargo install --git http://github.com/ekuinox/oauth2-helper.git
> oauth2-helper init <FILE_NAME> \
  [-a <OAUTH2_AUTH_URL>] \
  [-t <OAUTH2_TOKEN_URL>] \
  [-i <OAUTH2_CLIENT_ID>] \
  [-e <OAUTH2_CLIENT_SECRET>] \
  [-s <SCOPE...>]
> oauth2-helper auth <FILE_NAME>
authorize_url => [AUTHORIZE_URL]
ENTER REDIRECTD URL:
<REDIRECTED_URL HERE>
access_token: [ACCESS_TOKEN]
refresh_token: [REFRESH_TOKEN]
```
