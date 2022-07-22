# alacritty-driver

## About

alacritty-driver is an alacritty based driver. You can interact with alacritty in your program through TCP-based localsockets.

About alacritty : https://github.com/alacritty/alacritty

## Run Driver

```sh
.\alacritty_driver.exe --local-socket-port [port]
```

## Set Window Size

alacritty-driver will send packets in the following format to notify windows size changes

```
+-----------------+---------------------+---------------------+
| FLAG(0x37,0x37) |         ROWS        |         COLS        |
+-----------------+---------------------+---------------------+
|      u16        |        be_u16       |        be_u16       |
+-----------------+---------------------+---------------------+
```

## License

alacritty-driver license inherited from alacritty 

https://github.com/alacritty/alacritty/blob/master/LICENSE-APACHE