-- Your SQL goes here
create table todos (
  id serial not null
  , title character varying(255) not null
  , memo text
  , done BOOLEAN default 'f' not null
  , created_at timestamp default now() not null
  , updated_at timestamp default now() not null
  , constraint todos_PKC primary key (id)
) ;

comment on table todos is 'todos';
comment on column todos.id is 'ID';
comment on column todos.title is 'タイトル';
comment on column todos.memo is 'メモ';
comment on column todos.done is '完了';
comment on column todos.created_at is '作成日';
comment on column todos.updated_at is '更新日';

