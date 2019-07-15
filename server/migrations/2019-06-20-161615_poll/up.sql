-- Your SQL goes here

CREATE TYPE progress AS ENUM ('not_started', 'in_progress', 'finished');

CREATE TABLE polls (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  summary TEXT NOT NULL,
  full_description_link VARCHAR,
  poll_type VARCHAR NOT NULL,
  current_progress progress NOT NULL DEFAULT 'not_started',
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX polls_index ON polls(email);

CREATE TABLE proposals (
  id SERIAL PRIMARY KEY,
  summary TEXT NOT NULL,
  full_description_link VARCHAR,
  poll_id INTEGER NOT NULL REFERENCES polls(id),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX proposals_index ON proposals(poll_id);

CREATE TABLE user_invites (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL,
  poll_id INTEGER NOT NULL REFERENCES polls(id),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX user_invites_index ON user_invites(email, poll_id);

CREATE TABLE user_invite_locks (
  id SERIAL PRIMARY KEY,
  user_invite_id INTEGER NOT NULL REFERENCES user_invites(id),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX user_invite_locks_index ON user_invite_locks(user_invite_id);

CREATE TABLE votes (
  id SERIAL PRIMARY KEY,
  user_invite_id INTEGER NOT NULL REFERENCES user_invites(id),
  proposal_id INTEGER NOT NULL REFERENCES proposals(id),
  points DOUBLE PRECISION NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX votes_index ON votes(user_invite_id, proposal_id);