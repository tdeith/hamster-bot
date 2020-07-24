###hamster-bot

This project aims to control the innards of a hamster-ball robot. 
It is designed to run on SMT32DiscoveryF3 boards. 

The robot will be driven by two motors: 
 - One to drive the hamster-ball around a central axis, by rotating a 
pendulum about said central axis
 - And another motor to laterally tilt the pendulum, effectively
 tilting the hamsterball's axis, causing a turn during forward/
reverse drive.

The onboard sensors will eventually be:
 - The F3's onboard IMU for gravity/bump detection,
 - Four IR range finders, mounted _Somehow_ to peer outside the hamsterball in the four cardinal directions

Presently, the code is in a position to prove to @tdeith that he 
is successfully reading the IMU; and is prepared to drive 
motors (once they are specc'd) 
