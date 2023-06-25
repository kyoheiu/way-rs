# WAY

Simple auth server & dashboard written in Rust.\
This app works as the bridge between proxy server and LDAP server, using JWT cookie.

## Prerequisites
- proxy server e.g. nginx
- LDAP server e.g. lldap

## Deploy

1. Set your reverse proxy properly.
   ```
   # e.g. nginx auth_request setting
   # 127.0.0.1:8080 -> backend server
   # 127.0.0.1:9090 -> auth server (way)
   server_name  app.domain.com; # backend server URL

   location / {
       auth_request .auth;
       error_page 500 @auth_failed;
       proxy_pass http://127.0.0.1:8080;
   }

   location .auth {
       internal;
       proxy_pass http://127.0.0.1:9090/api/auth;
       proxy_pass_request_body off;
       proxy_pass_header Set-Cookie;
   }

   location @auth_failed {
       # Add ref param to redirect after logging in
       return 302 https://auth.domain.com?ref=$scheme://$http_host$request_uri;
   }
   ```

2. `git clone` this repo, and add `./config/.env`.
   ```
   WAY_DOMAIN=domain.com
   WAY_SECRET_KEY=secret_string
   WAY_NETWORK=docker_network_that_ldap_server_uses
   ```

   Optionally you can add `./config/config.yml` and use this app as a simple
   personal dashboard.

   ```
   - name: app1
     url: https://app1.domain.com
   - name: app2
     url: https://app2.domain.com
   ```

3. `sudo docker run -d --env-file ./config/.env -v ./config:/home/way/config --network="ldap_server_name" --name way -p 9090:9090 kyoheiudev/way-rs:0.2.0`
