CREATE TABLE IF NOT EXISTS _noot (
    schema_version text unique not null primary key,
    initialized integer unique
);

CREATE TABLE IF NOT EXISTS metadata (
    field text unique not null primary key,
    entry text
);

CREATE TABLE IF NOT EXISTS plugins (
    id text unique not null primary key,
    kind text not null default 2, -- The default kind of plugin will be a git plugin.
    ref text not null unique, -- If plugin is of kind 2, this will be a git url, if kind 1, a file path
    pin text default null, -- If plugin is of kind 2 this will be a commit sha
    enable integer default false
);

CREATE TABLE IF NOT EXISTS settings (
    id text unique not null primary key,
    value blob default null
);

CREATE TABLE IF NOT EXISTS assets (
    id text unique not null primary key,
    data BLOB not null,
    mime text not null
);

CREATE TABLE IF NOT EXISTS files (
    path text unique not null primary key,
    checksum text unique not null,
    size integer not null,
    ephemeral integer not null default false,
    buffer blob default null
);
