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

# 角色列表

角色：lucy-voice-m8

介绍：说书先生

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-m8.wav)
<hr>


角色：lucy-voice-f34

介绍：书香少女

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-f34.wav) 
<hr>


角色：lucy-voice-female2

介绍：暖心姐姐

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-female2.wav) 
<hr>


角色：lucy-voice-f36

介绍：温柔妹妹

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-f36.wav) 
<hr>


角色：lucy-voice-suxinjiejie

介绍：酥心御姐

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-suxinjiejie.wav)
<hr>


角色：lucy-voice-female1

介绍：邻家小妹

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-female1.wav)
<hr>


角色：lucy-voice-xueling

介绍：元气少女

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-xueling.wav)
<hr>


角色：lucy-voice-f37

介绍：文艺少女

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-f37.wav)
<hr>


角色：lucy-voice-guangdong-f1

介绍：东北老妹儿

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-guangdong-f1.wav)
<hr>


角色：lucy-voice-male3

介绍：憨厚老哥

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-male3.wav)
<hr>


角色：lucy-voice-silang

介绍：四郎

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-silang.wav)
<hr>


角色：lucy-voice-lvbu

介绍：吕布

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-lvbu.wav)
<hr>


角色：lucy-voice-m14

介绍：低沉男声

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-m14.wav)
<hr>


角色：lucy-voice-f38

介绍：傲娇少女

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-f38.wav)
<hr>


角色：lucy-voice-houge

介绍：猴哥

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-houge.wav)
<hr>


角色：lucy-voice-laibixiaoxin

介绍：小新

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-laibixiaoxin.wav)
<hr>


角色：lucy-voice-daji

介绍：妲己

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-daji.wav)
<hr>


角色：lucy-voice-male2

介绍：磁性大叔

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-male2.wav)
<hr>


角色：lucy-voice-guangxi-m1

介绍：广西大表哥

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-guangxi-m1.wav)
<hr>


角色：lucy-voice-lizeyan

介绍：霸道总裁

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-lizeyan.wav)
<hr>


角色：lucy-voice-male1

介绍：憨憨小弟

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-male1.wav)
<hr>


角色：lucy-voice-m101

介绍：爹系男友

试听：[播放](https://res.qpt.qq.com/qpilot/tts_sample/group/lucy-voice-m101.wav)
<hr>
