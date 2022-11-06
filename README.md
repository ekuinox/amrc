# oauth2-helper

## 使い方

```console
> cargo install --git http://github.com/ekuinox/oauth2-helper.git
> oauth2-helper init <PRESET_FILE_NAME> \
  [-a <OAUTH2_AUTH_URL>] \
  [-t <OAUTH2_TOKEN_URL>] \
  [-i <OAUTH2_CLIENT_ID>] \
  [-e <OAUTH2_CLIENT_SECRET>] \
  [-s <SCOPE...>]
> oauth2-helper auth <PRESET_FILE_NAME> <CONF_FILE_NAME>
authorize_url => [AUTHORIZE_URL]
ENTER REDIRECTD URL:
<REDIRECTED_URL HERE>
> oauth2-helper req <CONF_FILE_NAME> get \
  <RESOURCE_PATH>
[RESPONSE_TEXT]
```
