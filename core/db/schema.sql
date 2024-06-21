-- -- もし存在すれば以下２つのテーブルを削除する
-- -- デプロイ時に毎回削除するわけにはいかないので，コメントアウトする
DROP TABLE IF EXISTS Times;
DROP TABLE IF EXISTS Guilds;

DROP TABLE IF EXISTS discord_times;
DROP TABLE IF EXISTS discord_guilds;



CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    discord_user_id NUMERIC(20) UNIQUE,
    slack_user_id TEXT UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    token_random INT
);

-- guild_id等が数字で表せたのはDiscordの話で，Slackなどでは文字列で表される
-- そのため，文字列で表すように変更

CREATE TABLE IF NOT EXISTS discord_guilds (
    guild_id NUMERIC(20) NOT NULL,
    guild_name VARCHAR(255),
    PRIMARY KEY (guild_id)
);

-- kindは，DiscordかSlackかを表す
-- guildsテーブルにもあるが，扱いやすさのためにこちらにも持たせる
-- そうすれば，join操作をしなくてもkindを取得できる
CREATE TABLE IF NOT EXISTS discord_times (
    user_id NUMERIC(20) NOT NULL,
    guild_id NUMERIC(20) NOT NULL,
    user_name VARCHAR(255) NOT NULL,
    channel_id NUMERIC(20) NOT NULL,
    PRIMARY KEY (user_id, guild_id),
    FOREIGN KEY (guild_id) REFERENCES discord_guilds(guild_id),
    FOREIGN KEY (user_id) REFERENCES users(discord_user_id)
);

CREATE TABLE IF NOT EXISTS slack_guilds (
    guild_id TEXT NOT NULL,
    guild_name VARCHAR(255),
    PRIMARY KEY (guild_id)
);

CREATE TABLE IF NOT EXISTS slack_times (
    user_id TEXT NOT NULL,
    guild_id TEXT NOT NULL,
    user_name VARCHAR(255) NOT NULL,
    channel_id TEXT NOT NULL,
    PRIMARY KEY (user_id, guild_id),
    FOREIGN KEY (guild_id) REFERENCES slack_guilds(guild_id),
    FOREIGN KEY (user_id) REFERENCES users(slack_user_id)
);




-- 以下は古いスキーマ
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