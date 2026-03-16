-- Add up migration script here
create table users (
  id integer primary key autoincrement,
  username text not null unique,
  password_hash text not null,
  storage_quota_bytes integer not null default 10737418240, -- 10 GB
  storage_used_bytes integer not null default 0,
  created_at text not null default current_timestamp,
  updated_at text not null default current_timestamp,
  deleted_at text
);

create table folders (
  id integer primary key autoincrement,
  user_id integer not null,
  name text not null,
  parent_folder_id integer,
  created_at text not null default current_timestamp,
  updated_at text not null default current_timestamp,
  deleted_at text,
  foreign key (user_id) references users(id) on delete cascade,
  foreign key (parent_folder_id) references folders(id) on delete cascade
);

create table files (
  id integer primary key autoincrement,
  user_id integer not null,
  folder_id integer,
  name text not null,
  storage_path text not null,
  size_bytes integer not null,
  mime_type text not null,
  created_at text not null default current_timestamp,
  updated_at text not null default current_timestamp,
  deleted_at text,
  foreign key (user_id) references users(id) on delete cascade,
  foreign key (folder_id) references folders(id) on delete set null
);