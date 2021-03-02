CREATE OR REPLACE FUNCTION updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp(3),
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp(3),
		username TEXT UNIQUE NOT NULL,
		password TEXT NOT NULL,
		verified BOOLEAN NOT NULL,
    email TEXT UNIQUE NOT NULL
);
CREATE TRIGGER users_updated_at_modtime
BEFORE UPDATE ON users
FOR EACH ROW EXECUTE PROCEDURE updated_at_column();


CREATE TABLE refresh_tokens (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp(3),
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp(3),
		token TEXT NOT NULL,
		valid BOOLEAN NOT NULL,
		user_id INTEGER NOT NULL
);
CREATE TRIGGER refresh_token_updated_at_modtime
BEFORE UPDATE ON refresh_tokens
FOR EACH ROW EXECUTE PROCEDURE updated_at_column();

CREATE TABLE access_tokens (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp(3),
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp(3),
		token TEXT NOT NULL,
		refresh_token_id INTEGER NOT NULL
);
CREATE TRIGGER access_token_updated_at_modtime
BEFORE UPDATE ON access_tokens
FOR EACH ROW EXECUTE PROCEDURE updated_at_column();
