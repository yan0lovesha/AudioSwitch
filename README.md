# AudioSwitch
A tool built by Rust that can switch default audio playback device on windows.

## How to use
### specify which device you want to use
Execute it directly will list the active audio playback devices and you can input an index to switch to a the specified device.
PS D:\code\AudioSwitch\target\debug> .\audio_switch.exe
```
* 0 - Speakers (High Definition Audio Device)
  1 - Headset Earphone (Sennheiser BTD 800 USB)
Please input the index of above devices which you want to switch to:
1
Switched to your selected device:
  0 - Speakers (High Definition Audio Device)
* 1 - Headset Earphone (Sennheiser BTD 800 USB)
```

### Switch to next device
Execute it with a argument switch. It will switch to next device.
```
PS D:\code\AudioSwitch\target\debug> .\audio_switch.exe switch
  0 - Speakers (High Definition Audio Device)
* 1 - Headset Earphone (Sennheiser BTD 800 USB)
PS D:\code\AudioSwitch\target\debug> .\audio_switch.exe switch
* 0 - Speakers (High Definition Audio Device)
  1 - Headset Earphone (Sennheiser BTD 800 USB)
```


### Other
You can also create a shortcut to the command and pin it to your start menu or taskbar for easily switch. Or bind the command to some hotkey tool.
