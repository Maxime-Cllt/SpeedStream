-- Database initialization script
-- This runs when PostgreSQL container starts for the first time

-- Connect to the speedstream_db database
\c speedstream_db;

-- Create the SPEED table
CREATE TABLE SPEED
(
    ID         SERIAL PRIMARY KEY,
    SPEED      FLOAT4 NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);