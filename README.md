
Install with:
```sh
cargo install --path . --force --locked
```

Run:
```sh
hyprsunsetauto longitude latitude dayKelvin nightKelvin
```

Example:
```sh
hyprsunsetauto 53.5 9.7 6500 3000
```
In the ~/.config/hypr/hyprland.conf:
```
exec-once = hyprsunset -t 6500
exec-once = hyprsunsetauto 53.5 9.7 6500 3500
```
