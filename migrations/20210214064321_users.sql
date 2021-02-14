CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name TEXT UNIQUE NOT NULL,
    last_name TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp(3),
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp(3)
);

CREATE OR REPLACE FUNCTION updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER updated_at_modtime
BEFORE UPDATE ON users
FOR EACH ROW EXECUTE PROCEDURE updated_at_column();
