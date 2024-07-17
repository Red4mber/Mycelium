-- Add migration script here



-- CREATING AGENTS TABLE --

CREATE TABLE IF NOT EXISTS agents (
  id UUID NOT NULL PRIMARY KEY,
  first_ping TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_ping TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  address INET NOT NULL DEFAULT inet '0.0.0.0',
  notes VARCHAR(500)
);

INSERT INTO agents VALUES (gen_random_uuid(), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, inet '127.0.0.1', 'Test agent added manually');

-- CREATING OPERATORS TABLE --

-- Enum types in postgresql suck fat ass so I'm deleting everything,
--  fuck type safety, integers are my best friend now
--CREATE TYPE operator_role AS ENUM ('admin', 'operator', 'guest');

CREATE TABLE IF NOT EXISTS operators (
        id UUID NOT NULL PRIMARY KEY DEFAULT (gen_random_uuid()),
        name VARCHAR(100) NOT NULL,
        email VARCHAR(255) NOT NULL UNIQUE,
        password VARCHAR(100) NOT NULL,
        created_by UUID NOT NULL,
        role INTEGER NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        last_login TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
    );
CREATE INDEX operators_email_idx ON operators (email);

INSERT INTO operators (name, email, password, created_by, role) VALUES ('Melusine', 'fake@email.lol', 'oops i forgor', '00000000-0000-0000-0000-000000000000', 2);

