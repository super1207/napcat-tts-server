# NAPCAR-TTS-SERVER

这个软件可以利用 napcat 来暴露QQ的语音合成接口，返回wav文件，可以用来做语音合成。

# 配置文件

config.json 例子

```json
{
    "group_id":123456789,
    "host":"127.0.0.1",
    "port":7788,
    "napcat_token":"123456",
    "napcat_url":"http://127.0.0.1:3000",
}
```

group_id：随便一个机器人所在的QQ群号

host：要监听的ip地址，如果需要外网访问，请设置为`0.0.0.0`

port：要监听的端口号

napcat_token：napcat的token，可以在napcat的配置文件中找到

napcat_url：napcat的url，使用正向HTTP协议与 napcat 通信


# 访问

使用HTTP GET访问，例如：

```
http://127.0.0.1:7788/tts?character=lucy-voice-suxinjiejie&text=你好呀
```

character：角色，参见napcat的文档，可省略，默认为`lucy-voice-m8`。

text：要合成的文本，必填。
