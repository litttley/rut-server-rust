-- Your SQL goes here

CREATE TABLE users (
  id VARCHAR NOT NULL PRIMARY KEY,
  uname VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  join_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  avatar VARCHAR NOT NULL DEFAULT '',
  email VARCHAR NOT NULL DEFAULT '',
  intro TEXT NOT NULL DEFAULT '',
  UNIQUE (uname)
);

CREATE TABLE ruts (
  id VARCHAR NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  url VARCHAR NOT NULL DEFAULT '',
  content TEXT NOT NULL DEFAULT '',
  create_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  renew_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  author_id VARCHAR NOT NULL DEFAULT '',
  user_id VARCHAR NOT NULL,
  user_name VARCHAR NOT NULL DEFAULT '',
  credential VARCHAR NOT NULL DEFAULT '',
  item_count INTEGER NOT NULL DEFAULT '0',
  comment_count INTEGER NOT NULL DEFAULT '0',
  star_count INTEGER NOT NULL DEFAULT '0'
);

CREATE TABLE items (
  id VARCHAR NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  uiid VARCHAR NOT NULL DEFAULT '',
  authors VARCHAR NOT NULL DEFAULT '',
  pub_at VARCHAR NOT NULL DEFAULT '',
  publisher VARCHAR NOT NULL DEFAULT '',
  category VARCHAR(16) NOT NULL DEFAULT 'Book',
  url VARCHAR NOT NULL DEFAULT '',
  cover VARCHAR NOT NULL DEFAULT '',
  edition VARCHAR NOT NULL DEFAULT '',
  detail TEXT NOT NULL DEFAULT '',
  rut_count INTEGER NOT NULL DEFAULT '0',
  etc_count INTEGER NOT NULL DEFAULT '0',
  done_count INTEGER NOT NULL DEFAULT '0'
);

CREATE TABLE collects (
  id VARCHAR NOT NULL PRIMARY KEY,
  rut_id VARCHAR NOT NULL,
  item_id VARCHAR NOT NULL,
  item_order INTEGER NOT NULL,
  content TEXT NOT NULL DEFAULT '',
  user_id VARCHAR NOT NULL,
  collect_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (rut_id, item_id)
);

CREATE TABLE etcs (
  id VARCHAR NOT NULL PRIMARY KEY,
  content TEXT NOT NULL DEFAULT '',
  create_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  etc_id VARCHAR NOT NULL DEFAULT ''
);

CREATE TABLE tags (
  id VARCHAR NOT NULL PRIMARY KEY,
  tname VARCHAR UNIQUE NOT NULL,
  intro TEXT NOT NULL DEFAULT '',
  logo VARCHAR NOT NULL DEFAULT '',
  pname VARCHAR NOT NULL DEFAULT '',
  item_count INTEGER NOT NULL DEFAULT '0',
  rut_count INTEGER NOT NULL DEFAULT '0',
  etc_count INTEGER NOT NULL DEFAULT '0',
  star_count INTEGER NOT NULL DEFAULT '0',
  UNIQUE (tname, pname)
);

CREATE TABLE tagruts (
  id VARCHAR NOT NULL PRIMARY KEY,
  tname VARCHAR NOT NULL,
  rut_id VARCHAR NOT NULL,
  count INTEGER NOT NULL DEFAULT '1',
  UNIQUE (tname, rut_id)
);

CREATE TABLE tagitems (
  id VARCHAR NOT NULL PRIMARY KEY,
  tname VARCHAR NOT NULL,
  item_id VARCHAR NOT NULL,
  count INTEGER NOT NULL DEFAULT '1',
  UNIQUE (tname, item_id)
);

CREATE TABLE tagetcs (
  id VARCHAR NOT NULL PRIMARY KEY,
  tname VARCHAR NOT NULL,
  etc_id VARCHAR NOT NULL,
  UNIQUE (tname, etc_id)
);

CREATE TABLE starruts (
  id VARCHAR NOT NULL PRIMARY KEY,
  user_id VARCHAR NOT NULL,
  rut_id VARCHAR NOT NULL,
  star_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  note VARCHAR NOT NULL DEFAULT '',
  UNIQUE (user_id, rut_id)
);

CREATE TABLE startags (
  id VARCHAR NOT NULL PRIMARY KEY,
  user_id VARCHAR NOT NULL,
  tname VARCHAR NOT NULL,
  star_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  note VARCHAR NOT NULL DEFAULT '',
  UNIQUE (user_id, tname)
);

CREATE TABLE staritems (
  id VARCHAR NOT NULL PRIMARY KEY,
  user_id VARCHAR NOT NULL,
  item_id VARCHAR NOT NULL,
  star_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  note VARCHAR NOT NULL DEFAULT '',
  flag VARCHAR NOT NULL DEFAULT '',
  UNIQUE (user_id, item_id)
);
