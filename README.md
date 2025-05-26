Welcome to the feverbft wiki!

This project is part of the implementation for Master Thesis on the topic "FEVER Optimal Responsive View Synchronisation". This work is an extension of novel approach proposed by authors [Andrew Lewis-Pye, Ittai Abraham](https://arxiv.org/abs/2301.09881). 

# Implementation
## Build your Time-keeping device
![Raspberry Pi3 (left) and BeagleBone Black Wireless (right) with ubox NEO-6M gps modules](https://github.com/varuneranki/feverbft/blob/master/images/RaspiGPS-min.jpeg)
Raspberry Pi3 (left) and BeagleBone Black Wireless (right) with ubox NEO-6M gps modules

It requires one or more Time-keeping servers based on the accuracy requirements. Alternatively if less accuracy is enough, select your nearest NTP public servers (Network Time Protocol) and configure it in the `simple_get_time()` function in the `clocky.rs` file for all instances of peers (`peer-server, peer, peerb`).

`match sntpc::simple_get_time("time.google.com:123", socket.try_clone().unwrap()) {`

In the case of locally hosted GPS based time-keeping server with some type of crystal oscillator, then use the documentation provided by [chrony-project.org](https://chrony-project.org/examples.html#_client_using_local_server_and_hardware_timestamping)

A detailed tutorial is available from [austinsnerdythings.com](https://austinsnerdythings.com/2021/04/19/microsecond-accurate-ntp-with-a-raspberry-pi-and-pps-gps/) for [Raspberry Pi](https://www.raspberrypi.com/products/) based GPSDO with various types of ublox modules. There is a newer [`2025 implementation`](https://austinsnerdythings.com/2025/02/14/revisiting-microsecond-accurate-ntp-for-raspberry-pi-with-gps-pps-in-2025/) with latest hardware.

In short,
1. install the packages `pps-tools gpsd gpsd-clients python-gps chrony` using apt package manager in Terminal
2. configure the `/boot/config.txt` file to include the specific GPIO pin for 1PPS signal and enable UART serial interface with baud rate 9600.
3. add a new line to `/etc/modules` "`pps-gpio`"
4. connect the ublox GPS module based on the pin layout to Raspberry Pi's 40 pin header and enable to the serial port hardware to be accessible for GPS device
5. check if PPS signal is working and configure GPSd (GPS daemon) to start immediately upon boot
6. if everything is working perfectly, the GPS module could lock a set of 8-11 GPS satellites based on the weather conditions and placement of the antenna. Use `cgps` and `gpsmon` to check the status
7.  finally configure chrony to set the reference clock.
> `refclock SHM 0 refid NMEA offset 0.200`

> `refclock PPS /dev/pps0 refid PPS lock NMEA`


**These steps slightly vary based on your chosen hardware and use the documentation provided by the manufacturer for the hardware.**

## Feverbft algorithm implementation
There are three building blocks for the fever consensus algorithm. `peerserver, peer, peerb`. Use the docker file provided to deploy the rust crates.
### peer-server
it configures a docker network to accommodate the peers or nodes of the network.

Initially peer-server implementation (v0) has a browser based UI to deploy new peers and also select whether they would be a byzantine or a non byzantine peer for testing purposes. It is a cumbersome process to manage in the background using a JSON file and it worked intermittently. This approach was abandoned due to time constraints for the duration of the thesis project.

Open a new terminal and navigate to the inner folder structure for peer-server and build the docker file and use `docker-compose up` command to make an instance of peer-server. It would configure the docker network and you can check the status of the network using `docker-network` command

### peer and peerb
peer displays normal, non-byzantine behavior while peerb displays abnormal, byzantine behavior in the network.

Open a new terminal for each block and navigate to the inner folder structure for both and build the respective docker files. Use `docker-compose up` command to run instances of peer and peerb nodes. The number of nodes is configured in the respective docker compose files `docker-compose.yaml`. **Do not forget to save the compose file after making the changes.**

![configuration of peer and peerb](https://github.com/varuneranki/feverbft/blob/master/images/configuration.png)
configuration of 6 non byzantine peers and 6 byzantine peers

# Performing the consensus
YOU HAVE TO CHOOSE A LEADER: Use the docker UI and randomly open one of the running instances of peer or peerb and use the following commands.
START ATTACK to instruct everyone to attack.
START RETREAT to instruct everyone to retreat.
KLOCK to just obtain NTP data for testing purposes if chrony clock sychronisation is working.

Observations: peers repeat the same instruction to ATTACK or RETREAT when the leader instructs. Byzantine peers randomly decide to ATTACK or RETREAT irrespective of what the leader isntructs. For artifical test purposes, all instances of peerb respond with opposite of the leader's instruction. 

![6 non byzantine peers and 6 byzantine peers](https://github.com/varuneranki/feverbft/blob/master/images/6peer6peerb.png)
consensus of 6 non byzantine peers and 6 byzantine peers

