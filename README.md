1. Enable SPI using raspi-config
2. Add to /boot/config.txt the following:
spidev.bufsiz=32768
core_freq=250
core_freq_min=250 (not sure)

