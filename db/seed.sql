CREATE TABLE IF NOT EXISTS files (
    id text unique primary key not null,
    path text unique not null,
    hash text not null,
    last_mod integer not null
);

CREATE TABLE IF NOT EXISTS links (
    id text unique primary key not null,
    source text not null,
    file text not null,
    broken integer,
    last_checked integer not null
);

-- CREATE TABLE IF NOT EXISTS plugins (
--     id text unique primary key not null,
--     path
-- );

-- Trigger to automatically break links which linked to the deleted file
CREATE TRIGGER IF NOT EXISTS break_link_when_file_deleted
    AFTER DELETE ON files FOR EACH ROW BEGIN
        UPDATE links set broken = true, last_checked=unixepoch() where file = OLD.id;
end;


CREATE TABLE IF NOT EXISTS tags (
    id text unique primary key,
    tag text
);
