# Wokwi rust embassy examples

### Wokki examples from others
     * https://wokwi.com/projects/419546730269316097
     * https://wokwi.com/projects/391678800985971713
     * https://wokwi.com/projects/401636991823336449
     * https://wokwi.com/projects/421383217960391681

### Justfile

```sh
just --set bin hello-world esp32c3

#or if you have fzf installed

just --choose bin=hello-world
```

### esp-generate

   ```sh
esp-generate --chip esp32c3 --headless -o unstable-hal -o embassy -o log -o esp-backtrace -o vscode -o wokwi simple
   ```

### Inspiration from others

1. https://github.com/theembeddedrustacean/ser-no-std

2. https://github.com/esp-rs/esp-hal/tree/main/examples

