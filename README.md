# Challenge. Build your own Redis with Rust

It's my #100DaysOfCode. 

### GROW
- **Goal** -
Build Redis from scratch with Rust. Cause I want to write perfect program with Rust.

- **Reality** -
I have time to learn and have materials that will help me achieve my goal.

- **Options** - 
I'm newbie in Rust. To reach my goal I have to learn the basics of the Rust, [The Event Loop and Nonblocking IO](https://build-your-own.org/redis/05_event_loop_intro), basics of the TCP protocol and data structures that are used in the Redis.

- **Will** - 
I accept the challenge #100DaysOfCode. I will learn and build for a hundred days and I will record all my actions here every day. I will take the book([Build Your Own Redis with C/C++](https://build-your-own.org/redis/)) as the basis of the training plan.

Time: 100 days.  
Start: March 5, 2023(Monday).  
End: June 13, 2023(Tuesday).  

Every day I will write my plan for the day and what I did. One day - one commit.

Educational materials: 
- Book [Build Your Own Redis with C/C++](https://build-your-own.org/redis/)
- Learning platform [Code Crafters](https://app.codecrafters.io/courses/redis?track=rust)

#### Day #0
I chose a book, created a project repository, and wrote a README.md.

#### Day #1
Created echo server with Rust(TcpListener). Test by telnet:
```
➜ telnet 127.0.0.1 8080
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
hello
hello
world
world
Connection closed by foreign host.
```
