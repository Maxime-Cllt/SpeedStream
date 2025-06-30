# Procedure for initializing the PostgreSQL database

# 1. Create the database

```
CREATE DATABASE "postgres";
```

# 2. Create the user

```
CREATE USER
    "postgres"
    WITH
        LOGIN
        PASSWORD
        'postgres';
GRANT ALL PRIVILEGES ON DATABASE "postgres" TO "postgres";
```

# 3. Create the table

```
CREATE TABLE "postgres"."speed" (
    "id" SERIAL PRIMARY KEY,
    "speed" INTEGER NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```


