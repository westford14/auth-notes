-- add users with customer role
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
        'alice',
        'alice@mail.com',
        -- password: pswd1234, hash(pswd1234pjZKk6A8YtC8$9p&UIp62bv4PLwD7@dF)
        '7c44575b741f02d49c3e988ba7aa95a8fb6d90c0ef63a97236fa54bfcfbd9d51',
        'pjZKk6A8YtC8$9p&UIp62bv4PLwD7@dF',
        'true',
        'customer',
        now(),
        now()
    );
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
        'bob',
        'bob@mail.com',
        -- password: pswd1234, hash(pswd1234pjZKk6A8YtC8$9p&UIp62bv4PLwD7@dF)
        '7c44575b741f02d49c3e988ba7aa95a8fb6d90c0ef63a97236fa54bfcfbd9d51',
        'pjZKk6A8YtC8$9p&UIp62bv4PLwD7@dF',
        'true',
        'customer',
        now(),
        now()
    );