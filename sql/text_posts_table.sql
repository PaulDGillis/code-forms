CREATE TABLE text_posts (
    id SERIAL primary key,
    title varchar(300) not null,    
    content TEXT not null,
    username varchar(25) not null,
    foreign key (username) REFERENCES users(username)
);