# Procedure for initializing the PostgreSQL database

# 1. Create the database

```
CREATE DATABASE "postgres";
```

# 2. Create the table

```
CREATE TABLE "postgres"."speed" (
    "id" SERIAL PRIMARY KEY,
    "speed" DECIMAL(5, 2) NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```


