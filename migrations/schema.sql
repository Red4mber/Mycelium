


CREATE TABLE IF NOT EXISTS agents (
  id UUID PRIMARY KEY,
  last_ping TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  address INET,
  notes VARCHAR(500),
);

INSERT INTO agents VALUES (gen_random_uuid(), CURRENT_TIMESTAMP, '127.0.0.1', 'Test agent added manually');