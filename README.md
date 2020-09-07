# client_credentials_client
client_credentials_client is a API client that request parallel.

# Setup
## Install
You need install rust and run `cargo install`.

```bash
$ brew install rust
$ cargo install --path .
```
## Settings
### Config file setting
Change name client_credentials_client.yml.tmp to client_credentials_client.yml and setting auth and API information.
Setting client_credentials_client.yml as follows.

```yaml
# `oauth` sets the clients data. 
# These clients send requests to the API in parallel.
oauth:
  - name: client1
    # `env` is a flag that whether `client_id` and `client_credential` use environment variables.
    # When false, `client_id` and `client_secret` is raw value.
    env: false
    client_id: raw_id
    client_secret: raw_secret
  - name: client2
    # When true, `client_id` and `client_secret` use environment variable.
    # The following examples use the values of the CLIENT_ID and CLIENT_SECRET environment variables.
    env: true
    client_id: CLIENT_ID
    client_secret: CLIENT_SECRET
# `token` sets the data when request token end point
token:
  # `token_url` is a URL that can get access token.
  token_url: http://localhost/oauth/token
  scope: scope1 scope2
# `api` describes information at the time of request of API using client credential method
api:
  # `api_name` is a label required to identify the API that each client in turn requests
  - api_name: post api
    # `method` is HTTP METHOD. now this allow `post` and `get`.
    method: post
    # `content_type` is HTTP CONTENT_TYPE.
    content_type: application/json
    # `variable` section is setting api endpoint data and body data.
    # This section needs to be set for the name section set by `oauth`.
    variable:
       # this is same as an `oauth` `name`
       - name: client1
         url: http://localhost/path/to/api
         body: "{\"test\":\"client1\"}"
       - name: client2
         url: http://localhost/path/to/api
         body: "{\"test\":\"client2\"}"
```
## Execution
After writing the configuration file, you can send requests in parallel to the specified endpoints as follows.
```bash
$ client_credentials_client
START GET TOKEN:: client1
END GET TOKEN:: client1
START GET TOKEN:: client2
END GET TOKEN:: client2
START PARALLEL
START: client1 phase
START: client2 phase
SEND: client1 post api
SEND: client2 post api
RECV: client1 status=202 Accepted
RECV: client1 body={"status":"ok"}
RECV: client2 status=202 Accepted
RECV: client2 body={"status":"ok"}
END: client1 phase
END: client2 phase
END PARALLEL
```

If you want to set custom configuration files, you can set -c option.

```bash
$ client_credentials_client -c your/config/file/path
```

# License
This software is available as open source under the terms of the MIT License.
