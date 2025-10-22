# Lerpz AI

## Docker compose guide

- Select `ID tokens (used for implicit and hybrid flows)`.

- Make sure to add a web app and add the URL below.

```
https://lerpz.local/api/auth/callback/microsoft-entra-id
```

- Use mkcert to create certificates & copy the generated files to /certs.

```
mkcert lerpz.com api.lerpz.com
```

- Also add these domain to your hosts file.

```
# Lerpz
127.0.0.1 lerpz.local
127.0.0.1 api.lerpz.local
```

- Start the containers using docker compose.

```
docker compose up --build
```

## Test the chat endpoint

```
curl http://localhost:3001/api/chat -v -X GET -H "Content-Type: application/json" -d '{"message":"Tell me a joke!"}'
```
