CREATE TABLE ping (
    id SERIAL NOT NULL PRIMARY KEY,
    endpoint VARCHAR(32) NOT NULL,
    delay INT NOT NULL,
    success BOOLEAN NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('brt'::text, now())
);