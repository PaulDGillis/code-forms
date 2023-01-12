CREATE TABLE users (
    username varchar(25) primary key,
    password_hash varchar(255) not null,
);