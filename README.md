<!-- 画像を表示する -->
![icon](/images/icon3.png)


# UbiquiTimes-Cardiac
[UbiquiTimes](https://github.com/neruneruna7/UbiquiTimes)の派生プロジェクト

## 概要
Times(ソフトウェアエンジニア系のコミュニティにおいてよくある，内部Twitterのようなものだと思っている．由来は知らない)に書き込んだ内容を，他のDiscordサーバーにも同時に書き込むBot．

## 使い方
- Botをサーバーに導入する
- はじめにut_c_guild_initスラッシュコマンドを実行する
  - 1回だけでよい
- あなたのTimesであるチャンネルで，ut_c_times_setスラッシュコマンドを実行する
  - 1回だけでよい
  - user_name: 他サーバから拡散されてくるときに，そのサーバーで使う名前 なんでもよい
- ~UTプレフィックスコマンドを実行する
  - ut_c_times_releaseスラッシュコマンドも同じだが，プレフィックスコマンドをを推奨
```
~UT
拡散したい内容
```

例
```
~UT
一度生まれたものは，そう簡単には死なない
```

### 対応している拡散内容
- テキスト

画像などファイルには現在対応していない

## Botの導入
導入URL
```
https://discord.com/oauth2/authorize?client_id=1215172502519812137&permissions=536873984&scope=bot
```

パーミッションはわからない点が多いため，不要なパーミッションがついている可能性がある

現在要求するパーミッション
- Manage Webhooks
- Read Messages/Viwe Channels
- Send Messages