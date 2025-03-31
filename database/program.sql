CREATE TABLE IF NOT EXISTS workspaces (
    id text unique not null primary key,
    name text not null,
    disk_path text not null unique,
    last_accessed integer
);

CREATE TABLE IF NOT EXISTS settings (
    id text unique not null primary key,
    value blob default null,
    enabled integer default false
);