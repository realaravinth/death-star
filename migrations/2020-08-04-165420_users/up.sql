CREATE TABLE "users" (
	id SERIAL PRIMARY KEY,
	username VARCHAR(64) NOT NULL UNIQUE,
    user_uuid UUID NOT NULL,
    hash TEXT NOT NULL,
    email VARCHAR(120) UNIQUE,
    role VARCHAR(32) NOT NULL DEFAULT 'user',
    name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)
