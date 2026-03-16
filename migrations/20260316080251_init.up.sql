-- Add up migration script here
create table "files" (
  id integer primary key autoincrement,
  name text not null,
  path text not null,
  size integer not null,
  created_at text not null default current_timestamp,
  updated_at text not null default current_timestamp
);

create table "folders" (
  id integer primary key autoincrement,
  name text not null,
  path text not null,
  created_at text not null default current_timestamp,
  updated_at text not null default current_timestamp
);

create table "file_folder" (
  file_id integer not null,
  folder_id integer not null,
  primary key (file_id, folder_id),
  foreign key (file_id) references files(id) on delete cascade,
  foreign key (folder_id) references folders(id) on delete cascade
);