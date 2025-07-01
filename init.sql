-- Database initialization script
-- This runs when PostgreSQL container starts for the first time

-- Connect to the speedstream_db database
\c speedstream_db;

-- Create the speed table
CREATE TABLE speed
(
    id         SERIAL PRIMARY KEY,
    speed      FLOAT4 NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);