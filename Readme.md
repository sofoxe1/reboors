simple program to edit config files then reboot  
before using edit and move `config.ini.example` to `/etc/rebootrs/config.ini` 

#### lines in config files need to end with #reboot-<target> for example:
```
GRUB_HIDDEN_TIMEOUT=0 #rebootrs-grub_no_wait
GRUB_HIDDEN_TIMEOUT=5 #rebootrs-grub_wait
```
program will comment out first line (and uncomment second one) when `rebootrs grub_wait` is run 

#### features:
target autocompletion, `reboot.py t` will be autocomplete to `reboot.py target`  
`-i` regenerate initrd (for now only using dracut)  
`-g` regenerates grub config  
`-y` doesnt ask before rebooting  
