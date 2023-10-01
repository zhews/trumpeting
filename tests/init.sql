\c trumpeting;

CREATE TABLE public_key (
        username                TEXT,
        public_key              BYTEA,
        public_key_signature    BYTEA,
        PRIMARY KEY (username)
);
