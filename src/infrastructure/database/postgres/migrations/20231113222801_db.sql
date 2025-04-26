-- create users table
CREATE TABLE users (
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    username TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    password_salt TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    roles TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
-- populate users table
INSERT INTO users (
        username,
        email,
        password_hash,
        password_salt,
        active,
        roles,
        created_at,
        updated_at
    )
VALUES (
        'admin',
        'admin@admin.com',
        -- password: pswd1234, hash(pswd1234pjZKk6A8YtC8$9p&UIp62bv4PLwD7@dF)
        '7c44575b741f02d49c3e988ba7aa95a8fb6d90c0ef63a97236fa54bfcfbd9d51',
        'pjZKk6A8YtC8$9p&UIp62bv4PLwD7@dF',
        'true',
        'admin',
        now(),
        now()
    );