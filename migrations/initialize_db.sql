-- MIGRATION SCRIPT ONLY FOR TESTING / DEV ENVIRONMENT
-- Do not use this on an actual server

-- Creating email type

CREATE EXTENSION citext;
CREATE DOMAIN email AS citext
  CHECK ( value ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

-- CREATING AGENTS TABLE --

CREATE TABLE IF NOT EXISTS agents (
	id UUID NOT NULL PRIMARY KEY,
	first_ping TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	last_ping TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	host_id UUID NOT NULL,
	operator_id UUID NOT NULL,
	-- CONSTRAINT fk_host FOREIGN KEY(host_id) REFERENCES hosts(id),				
    -- CONSTRAINT fk_operator FOREIGN KEY(operator_id) REFERENCES operators(id),
	notes TEXT
);

INSERT INTO agents (id, first_ping, last_ping, host_id, operator_id, notes) VALUES ('51d10216-2daf-41eb-a9ca-a8da3a3cc924', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 'c6fb70b3-6d40-47ed-920c-1f205bc0f232', '15a374ef-0eda-4f01-9e2a-e1505ba60ed1', 'Test agent for debug purposes');

-- CREATING OPERATORS TABLE --

-- Enum types in postgresql suck so I removed the enum,
-- -- fuck type safety, integers are my best friend now

CREATE TYPE operator_role AS ENUM ('admin', 'operator', 'guest');

CREATE TABLE IF NOT EXISTS operators (
	id UUID NOT NULL PRIMARY KEY DEFAULT (gen_random_uuid()),
	name VARCHAR(255) NOT NULL,
	email VARCHAR(255) NOT NULL UNIQUE,
	password VARCHAR(255) NOT NULL,
	created_by UUID NOT NULL,
	role OPERATOR_ROLE NOT NULL DEFAULT 'operator',
	created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	last_login TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX operators_email_idx ON operators (email); 

INSERT INTO operators (id, name, email, password, created_by, role) VALUES ('15a374ef-0eda-4f01-9e2a-e1505ba60ed1', 'Melusine', 'melusine@mycelium.com', '$2b$12$AlzNYI/5W98RB4fjtJ9ZfeWfs1ikQPKvs2MGfh0ER3SmUoRJyei7u', '00000000-0000-0000-0000-000000000000', 'admin');


-- CREATING HOSTS TABLE --

CREATE TABLE IF NOT EXISTS hosts (
	id UUID NOT NULL PRIMARY KEY,
	agent UUID NOT NULL,
	hostname VARCHAR(255) NOT NULL,
	os VARCHAR(255) NOT NULL,
	known_users VARCHAR(255)[]  NOT NULL,
	external_ip INET NOT NULL DEFAULT inet '0.0.0.0',
	processor_number VARCHAR(255) NOT NULL,
	processor_id VARCHAR(255) NOT NULL,
	userdomain VARCHAR(255),
	notes TEXT
);

INSERT INTO hosts (id, agent, hostname, os, known_users, external_ip, processor_number, processor_id, userdomain, notes) 
VALUES ('c6fb70b3-6d40-47ed-920c-1f205bc0f232', '51d10216-2daf-41eb-a9ca-a8da3a3cc924', 'DESKTOP-F4K3PC', 'Win11 24H2', ARRAY ['Administrator', 'Melusine'], INET '129.64.112.197', 4, 'x86 Family 15 Model 2 Stepping 9, GenuineIntel', 'DESKTOP-DEADPC', 'Fake host with dummy info, added to test database');

-- CREATING HOSTS TABLE --

CREATE TABLE IF NOT EXISTS files (
	id UUID NOT NULL PRIMARY KEY,
	host_id UUID NOT NULL,
	filename VARCHAR(255) NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
)
