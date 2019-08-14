<div align="center">
<h1> Connect4Square </h1>

<h2> Connect 4 - but with squares instead of circles. </h2>

</div>

<h3> Introduction </h3>

My name is Haison Nguyen and I'm currently taking a class: CS410: Rust Programming. I'm currently working on a term project - Connect4Square - using Rust-wasm, which essentially is a web assembly pack that binds Rust and Javascript code together, resulting in a fully functional Rust web application. Connect 4 square is just like the well known version - Connect 4 - except that it's in a square matrix instead of circles.

<h3> How it works </h3>

The rules are simple:
1. Each player takes a turn choosing the position of their next move.
2. When a player accumulates 4 total squares of their color in a row  (horizontal, vertical or diagonal), they win!
3. If a game ends without a winner, both players lose. (Rule of thumb: If you're not first, you're last).


<h1>UPDATE August 14, 2019</h1>

I've slightly changed the overall tech stack I was working with when developing my project.

Initially, I was using rust-wasm to generate Javascript from a rust code translation. However, I felt like the scope of the project didn't quite fit with rust-wasm. So, I used my past web development experience with web request routing and web socket communication to implement a rust server that routes web requests and communicates with clients through web sockets. For routing, I used Nickel-rs which is inspired from Express.js, a JS web framework with which I've worked with in the past. For web socket communication, I used ws-rs, which had numerous examples and great documentation.

So, the game is still Connect4Square (connect4 in a matrix).

To start the game:

Open the folder in terminal and run
``` cargo run ```

This will start the serve that handles the game, web requests and web socket communication

Open a web browser (I developed this using Google Chrome)
and head on over to

[http://127.0.0.1:8081](http://127.0.0.1:8081)

to start the game.

<h3> How to play </h3>

Once the browser is opened, the first player can start by pressing on top most element on the page. When clicked, the color will change to either red or yellow (opposite of first color), this will indicate the next player's move. 

Play until there is a winner, the window will alert with a message announcing the winner.

<h3> Licensing </h3>

[MIT LICENSE](https://github.com/haisonnguyen/connect-4-square/blob/master/LICENSE_MIT)

[APACHE LICENSE](https://github.com/haisonnguyen/connect-4-square/blob/master/LICENSE_APACHE)