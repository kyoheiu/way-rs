# WAY

Stupidly simple auth server written in Rust.\
For authentication, way uses JWT and sets cookie.

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

2. `git clone` this repo, and add `.env`.
   ```
   WAY_DOMAIN=domain.com
   WAY_SECRET_KEY=secret_string
   WAY_SECRET_SUB=also_secret_string
   WAY_USERNAME=yourname
   WAY_PASSWORD=password
   ```

3. `sudo docker run -d --env-file .env --name way -p 9090:9090 $(sudo docker build -q .)`

## TODO

- dashboard feature included?
