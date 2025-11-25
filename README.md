# Lerpz Core

A general-purpose library containing shared utilities, helpers, and reusable
components used across Lerpz projects. This repository serves as a central hub
for common functionality, reducing code duplication and ensuring consistency
across services and applications. Includes data models, validation utilities,
formatting functions, and other tools that are frequently needed throughout the
organization.

## Project structure

```
lerpz-ai/
| - certs/                       /* Certificates  */
| - docs/                        /* Documentation */
| - crates/                      /* Rust crates   */
| - pacakges/                    /* Node packages */
| - svc/                         /* Services      */
|   | - portal                   /* NextJS app    */
|   | - portal-api               /* Axum Rest API */
```

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
