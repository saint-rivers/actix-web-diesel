-- Your SQL goes here

create table if not exists todos
(
  id uuid primary key default gen_random_uuid(),
  title text not null,
  content text not null,
  completed bool not null default false,
  created_at timestamp not null default now(),
  updated_at timestamp not null
);