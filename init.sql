-- Database initialization script
-- This runs when PostgreSQL container starts for the first time

-- Connect to the speedstream_db database
\c speedstream_db;

-- Create the SPEED table
CREATE TABLE speed
(
    id          SERIAL PRIMARY KEY,
    sensor_name VARCHAR(32) DEFAULT ''                                              NOT NULL, -- Name of the sensor sending the data
    speed       FLOAT4      DEFAULT 0.0                                             NOT NULL, -- Speed in km/h
    lane        INT4        DEFAULT 0                                               NOT NULL, -- Lane (left, right, etc.)
    created_at  TIMESTAMPTZ DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'Europe/Paris') NOT NULL  -- Timestamp of the record creation
);