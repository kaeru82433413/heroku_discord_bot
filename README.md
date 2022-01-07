
herokuでRust・serenityのdiscordbotを動かせることを確認するためのプロジェクトです
herokuのBuildpacksにhttps://github.com/emk/heroku-buildpack-rust.gitを追加してあります

## 機能
環境変数"discord_bot_token"によってログインし、起動直後にbotのオーナーにDMを送ります。