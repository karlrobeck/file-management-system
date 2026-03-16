-- Add up migration script here
create table "files" (
  id integer primary key autoincrement,
  name text not null,
  path text not null,
  size integer not null,
  created_at text not null default current_timestamp,
  updated_at text not null default current_timestamp
);