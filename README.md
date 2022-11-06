# amrc

OAuth2 のトークンを拾ってきたり、それを使ってリクエストしたりするツール

## 使い方

```console
> cargo install --git https://github.com/ekuinox/amrc.git
> amrc init <PRESET_FILE_NAME> \
  [-a <OAUTH2_AUTH_URL>] \
  [-t <OAUTH2_TOKEN_URL>] \
  [-i <OAUTH2_CLIENT_ID>] \
  [-e <OAUTH2_CLIENT_SECRET>] \
  [-s <SCOPE...>]
> amrc auth <PRESET_FILE_NAME> <CONF_FILE_NAME>
authorize_url => [AUTHORIZE_URL]
ENTER REDIRECTD URL:
<REDIRECTED_URL HERE>
> amrc req <CONF_FILE_NAME> get \
  <RESOURCE_PATH>
[RESPONSE_TEXT]
```
