# HabForge

This project is a simple Rust application that periodically updates an openHAB item with the current illuminance value from a TinkerForge Ambient Light V2 Bricklet. The application can run either in standalone mode or as a systemd service.

## Systemd Configuration

Create a new systemd service configuration file called habforge.service in the /etc/systemd/system directory with the following content:

```
[Unit]
Description=Ambient Light Sensor to openHAB Service
After=network.target

[Service]
Type=notify
User=your_user
WorkingDirectory=/path/to/habforge
ExecStart=/path/to/habforge/habforge
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Replace your_user with your Linux user, /path/to/habforge with the path to the habforge folder, and /path/to/habforge/habforge with the path to your compiled Rust binary.

### Command Line for Systemd Service Setup

To set up the systemd service, run the following commands:

Reload the systemd configuration:

    sudo systemctl daemon-reload

Enable the new service to start automatically on boot:

    sudo systemctl enable habforge

Start the service:

    sudo systemctl start habforge

Check the status of the service:

    sudo systemctl status habforge

You should now have a running systemd service that updates the openHAB item with the current illuminance value from the TinkerForge Ambient Light V2 Bricklet.

### Configuration

To configure the Rust application using a configuration file, create a new file named config.toml in the /etc/habforge directory. This file will contain the necessary settings for the application to connect to the TinkerForge Ambient Light V2 Bricklet and the openHAB server.

```
host = "127.0.0.1"
port = 4223
uid = "your_uid"
item = "your_openhab_item_name"
openhab_url = "http://your_openhab_server:8080/rest/items/"
threshold = 100
```

Replace the following placeholders with your specific values:

- your_uid: The UID of your TinkerForge Ambient Light V2 Bricklet.
- your_openhab_item_name: The name of the openHAB item that you want to update with the illuminance value.
- your_openhab_server: The IP address or hostname of your openHAB server.

You may change port and threshold if needed. Values should be fine.



### Additional Note: Setting up Brickd

To use the TinkerForge Ambient Light V2 Bricklet, you may need to set up the Brick Daemon (brickd) to establish communication between the Bricklet and your computer. Brickd acts as a proxy between the USB interface of the Bricks and the API bindings.

Follow the instructions provided in the TinkerForge Brickd documentation to install and configure brickd for your specific operating system: [TinkerForge Brickd](https://www.tinkerforge.com/en/doc/Software/Brickd.html)

After installing and configuring brickd, ensure that it is running and properly configured to work with your TinkerForge Ambient Light V2 Bricklet. This will ensure a seamless integration with the habforge Rust application and allow it to periodically update the openHAB item with the current illuminance value from the Bricklet.


