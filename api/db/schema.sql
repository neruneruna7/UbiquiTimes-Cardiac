-- もし存在すれば以下２つのテーブルを削除する
DROP TABLE IF EXISTS Times;
DROP TABLE IF EXISTS Guilds;


-- Rustのu64型を格納するためにNUMERIC(20)を使用


CREATE TABLE IF NOT EXISTS Guilds (
    guild_id NUMERIC(20) NOT NULL,
    guild_name VARCHAR(255),
    PRIMARY KEY (guild_id)
);


CREATE TABLE IF NOT EXISTS Times (
    user_id NUMERIC(20) NOT NULL,
    guild_id NUMERIC(20) NOT NULL,
    user_name VARCHAR(255) NOT NULL,
    channel_id NUMERIC(20) NOT NULL,
    webhook_url TEXT NOT NULL,
    PRIMARY KEY (user_id, guild_id),
    FOREIGN KEY (guild_id) REFERENCES Guilds(guild_id)
);