# Wokwi rust embassy examples

### Problems
1. Hello world not working - prints out only once in each task, but then that is it.
   
   Working examples from wokwi projects online:

     * https://wokwi.com/projects/419546730269316097
     * https://wokwi.com/projects/391678800985971713
     * https://wokwi.com/projects/401636991823336449
     * https://wokwi.com/projects/421383217960391681

   They dont use the latest 1.0 beta version of esp-hal(use v0.22 and v0.21 respectively).

   Use the first one as a reference. See if you can get it working in VSCode first with the version
   of crates in in Cargo.toml

   ### esp-generate

   ```sh
   esp-generate --chip esp32c3 --headless -o unstable-hal -o probe-rs -o defmt -o panic-rtt-target -o vscode -o wokwi  simple 
   ```