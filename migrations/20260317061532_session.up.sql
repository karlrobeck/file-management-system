-- Add up migration script here
create table sessions (
  id integer primary key autoincrement,
  user_id integer not null,
  token text not null,
  expires_at datetime not null,
  created_at datetime not null default current_timestamp,
  updated_at datetime not null default current_timestamp,
  foreign key (user_id) references users(id) on delete cascade
);