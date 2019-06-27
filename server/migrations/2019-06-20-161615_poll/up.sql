-- Your SQL goes here

CREATE TYPE progress AS ENUM ('not_started', 'in_progress', 'finished');

CREATE TABLE poll (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  poll_type VARCHAR NOT NULL,
  current_progress progress NOT NULL DEFAULT 'not_started',
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX poll_index ON poll(email);

CREATE TABLE proposal (
  id SERIAL PRIMARY KEY,
  summary TEXT NOT NULL,
  full_description_link VARCHAR,
  poll_id INTEGER NOT NULL REFERENCES poll(id),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX proposal_index ON proposal(poll_id);

CREATE TABLE user_invite (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL,
  poll_id INTEGER NOT NULL REFERENCES poll(id),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX user_invite_index ON user_invite(email, poll_id);

CREATE TABLE vote (
  id SERIAL PRIMARY KEY,
  user_invite_id INTEGER NOT NULL REFERENCES user_invite(id),
  proposal_id INTEGER NOT NULL REFERENCES proposal(id),
  -- intended for votes to range from 0 - 100 with up to 4 decimal points of additional precision
  points NUMERIC(7, 4) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX vote_index ON vote(user_invite_id, proposal_id);